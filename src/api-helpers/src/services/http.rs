use eyre::Result;
use actix_web::{
  HttpRequest,
  HttpResponse,
};
use serde::Serialize;
use ticketland_core::error::Error;

pub fn internal_server_error(error: Option<Error>) -> Result<HttpResponse, Error> {
  Err(error.map_or( Error::GenericError("Internal Server Error".to_owned()),Into::<_>::into))
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

#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
  pub count: usize,
  pub skip: i64,
  pub limit: i64,
  pub result: T,
}

#[macro_export]
macro_rules! QueryString {
  ($pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
		#[derive(Deserialize,Debug, Clone)]
    $pub struct $name {
      pub skip: Option<i64>,
      pub limit: Option<i64>,
      $($fpub $field : $type,)*
    }

    impl QueryStringTrait for $name {
      fn skip(&self) -> Option<i64> { self.skip }
      fn limit(&self) -> Option<i64> { self.limit }
    }
  }
}

pub fn create_read_response<T: Serialize>(result: Result<Vec<T>>, skip: i64, limit: i64) -> Result<HttpResponse, Error> {
  result
  .map(|result| {
    HttpResponse::Ok()
      .json(BaseResponse {
        count: result.len(),
        result,
        skip: skip,
        limit: limit,
      })
  })
  .map_err(|error| error.into())
}

pub fn create_write_response(result: Result<()>) -> Result<HttpResponse, Error> {
  result
  .map(|_| HttpResponse::Ok().finish())
  .map_err(|error| error.into())
}

pub fn create_response<T: Serialize>(result: Result<T>) -> Result<HttpResponse, Error> {
  result
  .map(|result| HttpResponse::Ok().json(result))
  .map_err(|error| error.into())
}
