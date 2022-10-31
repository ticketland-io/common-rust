use std::{
  future::{ready, Ready as StdReady},
  sync::Arc,
  rc::Rc,
};
use actix::prelude::*;
use chrono::{Utc, TimeZone};
use eyre::{Result, Report, ContextCompat};
use actix_web::{
  HttpMessage,
  dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
  FromRequest,
  Error,
  HttpRequest,
  error::ErrorUnauthorized,
};
use futures_util::future::{LocalBoxFuture, ok, err, Ready};
use common_data::{
  helpers::{send_read},
  models::api_client::ApiClient,
  repositories::api_client::read_api_client,
};
use ticketland_crypto::{
  symetric::hmac::sign_sha256,
};
use ticketland_core::actor::neo4j::Neo4jActor;

const MAX_TOKEN_VALIDITY_SECS: i64 = 100000; // 60 secs;

#[derive(Debug, Clone)]
pub struct ClientAuth {
  pub client_id: String,
}

impl FromRequest for ClientAuth {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    req.extensions()
    .get::<ClientAuth>()
    .map(|client_auth| client_auth.clone())
    .map(ok)
    .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
  }
}
pub struct HmacAuthnMiddlewareFactory {
  neo4j: Arc<Addr<Neo4jActor>>,
}

impl HmacAuthnMiddlewareFactory {
  pub fn new(neo4j: Arc<Addr<Neo4jActor>>) -> Self {
    Self {neo4j}
  }
}

impl<S, B> Transform<S, ServiceRequest> for HmacAuthnMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HmacAuthnMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(HmacAuthnMiddleware {
        service: Rc::new(service),
        neo4j: self.neo4j.clone(),
      }))
    }
}

pub struct HmacAuthnMiddleware<S> {
  service: Rc<S>,
  neo4j: Arc<Addr<Neo4jActor>>,
}

impl<S, B> HmacAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static 
{
  fn is_valid_ts(ts: &str) -> Result<()> {
    let ts = Utc.timestamp(ts.parse()?, 0).time();
    let now = Utc::now().time();
    let diff = now - ts;

    if diff.num_seconds() >= MAX_TOKEN_VALIDITY_SECS {
      return Err(Report::msg("Unauthorized"));
    }

    Ok(())
  }

  async fn verify_sig(neo4j: Arc<Addr<Neo4jActor>>, msg: &str, sig: &str) -> Result<String> {
    let parts = msg.split(":").collect::<Vec<_>>();
    let client_id = parts.get(0).context("Unauthorized")?;
    let ts = parts.get(1).context("Unauthorized")?;
    
    Self::is_valid_ts(ts)?;
    
    let (query, db_query_params) = read_api_client(client_id.to_string());
    let api_client = send_read(Arc::clone(&neo4j), query, db_query_params)
    .await
    .map(TryInto::<ApiClient>::try_into)??;

    let local_sig = sign_sha256(&api_client.client_secret, msg)?;

    if sig != local_sig {
      return Err(Report::msg("Unauthorized"));
    }

    Ok(client_id.to_string())
  }
}

impl<S, B> Service<ServiceRequest> for HmacAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();
    let neo4j = Arc::clone(&self.neo4j);

    Box::pin(
      async move {
        let headers = req.headers();
        let msg = headers.get("X-TL-HMAC-AUTHOTIZATION-MSG").ok_or(ErrorUnauthorized("Unauthorized"))?
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        let token = headers.get("Authorization").ok_or(ErrorUnauthorized("Unauthorized"))?;
        let mut iter = token
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?
        .split_whitespace();
        
        if let Some(prefix) = iter.next() {
          if prefix != "TL-HMAC" {
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }

        let access_token = if let Some(access_token) = iter.next() {
          access_token
        } else {
          return Err(ErrorUnauthorized("Unauthorized"))
        };

        let client_id = Self::verify_sig(neo4j, msg, access_token)
        .await
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        req.extensions_mut().insert(ClientAuth {client_id});

        return Ok(srv.call(req).await?)
      }
    )
  }
}
