use actix_web::{HttpResponse};
use common::{
  error::Error,
};

pub fn internal_server_error(_error: Option<Error>) -> HttpResponse {
  HttpResponse::InternalServerError()
  .reason("500")
  .body("")
}

pub fn unauthorized_error() -> HttpResponse {
  HttpResponse::Unauthorized()
  .reason("401")
  .finish()
}
