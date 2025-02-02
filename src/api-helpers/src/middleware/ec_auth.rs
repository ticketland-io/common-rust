use std::{
  future::{ready, Ready as StdReady},
  rc::Rc,
};
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
use ticketland_crypto::{
  encoding::base64,
  asymetric::ed25519,
};

const MAX_TOKEN_VALIDITY_SECS: i64 = 60; // 60 secs;

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
pub struct EcAuthnMiddlewareFactory {}

impl<S, B> Transform<S, ServiceRequest> for EcAuthnMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = EcAuthnMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(EcAuthnMiddleware {
        service: Rc::new(service),
      }))
    }
}

pub struct EcAuthnMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> EcAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static 
{
  fn is_valid_ts(ts: &str) -> Result<()> {
    #[allow(deprecated)]
    let ts = Utc.timestamp(ts.parse()?, 0).time();
    let now = Utc::now().time();
    let diff = now - ts;

    if diff.num_seconds() >= MAX_TOKEN_VALIDITY_SECS {
      return Err(Report::msg("Unauthorized"));
    }

    Ok(())
  }

  fn verify_sig(msg: &str, sig: &str) -> Result<String> {
    let parts = msg.split(":").collect::<Vec<_>>();
    let client_id = parts.get(0).context("Unauthorized")?;
    let pub_key = base64::decode(client_id)?;
    let ts = parts.get(1).context("Unauthorized")?;

    Self::is_valid_ts(ts)?;
    ed25519::verify(msg.as_bytes(), &pub_key, sig)?;

    Ok(client_id.to_string())
  }
}

impl<S, B> Service<ServiceRequest> for EcAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();

    Box::pin(
      async move {
        let headers = req.headers();
        let msg = headers.get("X-TL-EC-AUTHOTIZATION-MSG").ok_or(ErrorUnauthorized("Unauthorized"))?
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        let token = headers.get("Authorization").ok_or(ErrorUnauthorized("Unauthorized"))?;
        let mut iter = token
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?
        .split_whitespace();
        
        if let Some(prefix) = iter.next() {
          if prefix != "TL-EC" {
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }

        let access_token = if let Some(access_token) = iter.next() {
          access_token
        } else {
          return Err(ErrorUnauthorized("Unauthorized"))
        };

        let client_id = Self::verify_sig(msg, access_token).map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        req.extensions_mut().insert(ClientAuth {client_id});

        return Ok(srv.call(req).await?)
      }
    )
  }
}
