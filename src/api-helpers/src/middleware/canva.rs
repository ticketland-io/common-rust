use std::{
  future::{ready, Ready as StdReady},
  rc::Rc,
  str,
};
use actix_http::h1::Payload;
use actix_web::{
  HttpMessage,
  http::Method,
  web::{self},
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error,
  error::ErrorUnauthorized,
};
use chrono::{Utc};
use futures_util::{
  stream::StreamExt,
  future::{LocalBoxFuture},
};
use qstring::QString;

const LENIENCY_IN_SECS: i64 = 300;
const VERSION: &str = "v1";

const PATHS: [&str; 3] = [
  "/canva/configuration",
  "/canva/publish/resources/upload",
  "/canva/auth",
];

pub struct CanvaMiddlewareFactory {
  canva_key: String,
}

impl CanvaMiddlewareFactory {
  pub fn new(canva_key: String) -> Self {
    Self {canva_key}
  }
}

pub struct CanvaMiddleware<S> {
  service: Rc<S>,
  canva_key: String,
}

impl<S, B> Transform<S, ServiceRequest> for CanvaMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CanvaMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(CanvaMiddleware {
        service: Rc::new(service),
        canva_key: self.canva_key.clone(),
      }))
    }
}

impl<S, B> CanvaMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static 
{

  async fn create_get_message(req: &mut ServiceRequest) -> Result<(Vec<String>, String), Error> {
    let qs = QString::from(req.query_string());
    let result = qs.get("extensions")
      .and_then(|extensions| {
        qs.get("user").map(|user| (extensions, user))
      })
      .and_then(|(extensions, user)| {
        qs.get("time").map(|time| (extensions, user, time))
      })
      .and_then(|(extensions, user, time)| {
        qs.get("brand").map(|brand| (extensions, user, time, brand))
      })
      .and_then(|(extensions, user, time, brand)| {
        qs.get("state").map(|state| (extensions, user, time, brand, state))
      })
      .and_then(|(extensions, user, time, brand, state)| {
        qs.get("signatures").map(|signatures| (extensions, user, time, brand, state, signatures))
      });

    if result.is_none() {
      return Err(ErrorUnauthorized("Unauthorized"))
    }

    let (
      extensions,
      user,
      time,
      brand,
      state,
      signatures
    ) = result.unwrap();

    let signatures: Vec<String> = signatures.split(",").map(|s| s.to_owned()).collect();
    let message = format!("{}:{}:{}:{}:{}:{}", VERSION, time, user, brand, extensions, state);
    
    Ok((signatures, message))
  }

  async fn create_post_message(req: &mut ServiceRequest, path: &str) -> Result<(Vec<String>, String), Error> {
    let headers = req.headers();
    let result = headers.get("X-Canva-Timestamp")
      .and_then(|ts| {
        ts.to_str().ok().map(|ts| ts)
      })
      .and_then(|ts| {
        headers.get("X-Canva-Signatures").map(|signatures| (ts, signatures))
      })
      .and_then(|(ts, signatures)| {
        signatures.to_str().ok().map(|signatures| (ts, signatures))
      });

    if result.is_none() {
      return Err(ErrorUnauthorized("Unauthorized"))
    }

    let (ts, signatures) = result.unwrap();
    let ts = ts.to_owned();
    let signatures: Vec<String> = signatures.split(",").map(|s| s.to_owned()).collect();

    if Utc::now().timestamp() - ts.parse::<i64>().unwrap() > LENIENCY_IN_SECS {
      return Err(ErrorUnauthorized("Unauthorized"))
    }

    let mut body = req.take_payload();
    let mut raw_body = web::BytesMut::new();

    while let Some(item) = body.next().await {
      raw_body.extend_from_slice(&item?);
    }

    let raw_body_copy = std::str::from_utf8(&raw_body).unwrap();
    let message = format!("{}:{}:{}:{}", VERSION, ts, path.replace("/canva", ""), raw_body_copy);

    // We need to put back the body we just consumer so it is later available in the handler
    let (_, mut payload) = Payload::create(true);
    payload.unread_data(raw_body.into());
    req.set_payload(payload.into());

    Ok((signatures, message))
    
  }
}

impl<S, B> Service<ServiceRequest> for CanvaMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, mut req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();
    let canva_key = self.canva_key.clone();
    
    Box::pin(
      async move {
        let path = PATHS.into_iter().find(|p| *p == req.path());
        if path.is_none() {
          return Err(ErrorUnauthorized("Unauthorized"))
        }

        let (signatures, message);
        
        if req.method() == Method::POST {
          (signatures, message) = Self::create_post_message(&mut req, path.unwrap()).await?;
        } else if req.method() == Method::GET {
          (signatures, message) = Self::create_get_message(&mut req).await?;
        } else {
          // The middleware support only GET and POST requests
          return Err(ErrorUnauthorized("Unauthorized"))
        }

        let sig = Self::calculate_sig(canva_key, message);
      
        if !signatures.iter().any(|v| *v == sig) {
          return Err(ErrorUnauthorized("Unauthorized"))
        }
          
        return Ok(srv.call(req).await?)
      }
    )
  }
}
