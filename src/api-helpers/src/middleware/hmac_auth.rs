use std::{
  future::{ready, Ready as StdReady},
  rc::Rc,
  future::Future,
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
  symetric::hmac::sign_sha256,
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

// pub type ApiClientReader = Arc<Box<dyn FnOnce(String) -> Pin<Box<dyn Future<Output = Result<String>>>>>>;

pub struct HmacAuthnMiddlewareFactory<F, Fut>
where
  F: FnOnce(String) -> Fut + Copy + 'static,
  Fut: Future<Output = Result<String>>,
{
  pub api_client_reader: F,
}

impl <F, Fut> HmacAuthnMiddlewareFactory<F, Fut>
where
  F: FnOnce(String) -> Fut + Copy + 'static,
  Fut: Future<Output = Result<String>>,
{
  pub fn new(api_client_reader: F) -> Self {
    Self {api_client_reader}
  }
}

impl<F, Fut, S, B> Transform<S, ServiceRequest> for HmacAuthnMiddlewareFactory<F, Fut>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    F: FnOnce(String) -> Fut + Copy + 'static,
    Fut: Future<Output = Result<String>>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HmacAuthnMiddleware<F, Fut, S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(HmacAuthnMiddleware {
        service: Rc::new(service),
        api_client_reader: self.api_client_reader,
      }))
    }
}

pub struct HmacAuthnMiddleware<F, Fut, S>
where
  F: FnOnce(String) -> Fut + Copy + 'static,
  Fut: Future<Output = Result<String>>,
{
  service: Rc<S>,
  api_client_reader: F,
}

impl<F, Fut, S, B> HmacAuthnMiddleware<F, Fut, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    F: FnOnce(String) -> Fut + Copy + 'static,
    Fut: Future<Output = Result<String>>,
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

  async fn verify_sig(api_client_reader: F, msg: &str, sig: &str) -> Result<String> {
    let parts = msg.split(":").collect::<Vec<_>>();
    let client_id = parts.get(0).context("Unauthorized")?;
    let ts = parts.get(1).context("Unauthorized")?;
    
    Self::is_valid_ts(ts)?;
    
    let client_secret = api_client_reader(client_id.to_string()).await?;
    let local_sig = sign_sha256(&client_secret, msg)?;

    if sig != local_sig {
      return Err(Report::msg("Unauthorized"));
    }

    Ok(client_id.to_string())
  }
}

impl<F, Fut, S, B> Service<ServiceRequest> for HmacAuthnMiddleware<F, Fut, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    F: FnOnce(String) -> Fut + Copy + 'static,
    Fut: Future<Output = Result<String>>,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();
    let api_client_reader = self.api_client_reader;

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

        let client_id = Self::verify_sig(api_client_reader, msg, access_token)
        .await
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        req.extensions_mut().insert(ClientAuth {client_id});

        return Ok(srv.call(req).await?)
      }
    )
  }
}
