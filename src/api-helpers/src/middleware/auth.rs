use std::{
  future::{ready, Ready as StdReady},
  rc::Rc,
};
use actix_web::{
  web,
  HttpMessage,
  dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
  FromRequest,
  Error,
  HttpRequest,
  error::ErrorUnauthorized,
};
use futures_util::future::{LocalBoxFuture, ok, err, Ready};
use fireauth::api::User;
use crate::{
  utils::store::Store,
};

#[derive(Debug, Clone)]
pub struct AuthData {
  pub user: User,
}

impl FromRequest for AuthData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    req.extensions()
      .get::<AuthData>()
      .map(|auth_data| auth_data.clone())
      .map(ok)
      .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
  }
}

pub struct AuthnMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthnMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthnMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(AuthnMiddleware {
        service: Rc::new(service),
      }))
    }
}

pub struct AuthnMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, mut req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();
    
    Box::pin(
      async move {
        let store = req.extract::<web::Data<Store>>().await;
        let headers = req.headers();
        let bearer = headers.get("Authorization");
        
        if bearer.is_none() {
          return Err(ErrorUnauthorized("Unauthorized"))
        }
        
        let mut iter = bearer.unwrap().to_str().unwrap().split_whitespace();
        
        if let Some(prefix) = iter.next() {
          if prefix != "Bearer" {
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }

        let access_token = if let Some(access_token) = iter.next() {
          access_token
        } else {
          return Err(ErrorUnauthorized("Unauthorized"))
        };
        
        match store.unwrap().firebase_auth.get_user_info(&access_token).await {
          Ok(user) => {
            // make the user available to the downstream handlers
            req.extensions_mut().insert(AuthData {user});
  
            return Ok(srv.call(req).await?)
          },
          Err(error) => {
            println!("{}", error);
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }
      }
    )
  }
}
