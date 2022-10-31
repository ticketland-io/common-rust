use std::{
  future::{ready, Ready as StdReady},
  rc::Rc,
};
use actix_web::{
  HttpMessage,
  dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
  FromRequest,
  Error,
  HttpRequest,
  error::ErrorUnauthorized,
};
// use ticketland_core::error::Error;
use futures_util::future::{LocalBoxFuture, ok, err, Ready};

pub struct HmacAuthnMiddlewareFactory {}

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
      }))
    }
}

pub struct HmacAuthnMiddleware<S> {
  service: Rc<S>,
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

    Box::pin(
      async move {
        let headers = req.headers();
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

        let _access_token = if let Some(access_token) = iter.next() {
          access_token
        } else {
          return Err(ErrorUnauthorized("Unauthorized"))
        };

        //TODO: validate the access token

        return Ok(srv.call(req).await?)
      }
    )
  }
}
