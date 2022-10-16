use actix_web::{
  HttpRequest,
  HttpResponse,
};
use ticketland_core::{
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

pub fn bad_request_error() -> HttpResponse {
  HttpResponse::BadRequest()
  .reason("400")
  .finish()
}

pub fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
	req.headers()
    .get(key)?
    .to_str()
    .ok()
}
