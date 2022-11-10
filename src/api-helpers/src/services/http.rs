use std::error::Error;
use actix_web::{
  HttpRequest,
  HttpResponse,
};
use serde::Serialize;

pub fn internal_server_error<E: Error>(_error: Option<E>) -> HttpResponse {
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

#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
  pub count: usize,
  pub skip: Option<u32>,
  pub limit: Option<u32>,
  pub result: T,
}

#[macro_export]
macro_rules! QueryString {
  ($pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
		#[derive(Deserialize,Debug, Clone)]
    $pub struct $name {
      pub skip: Option<u32>,
      pub limit: Option<u32>,
      $($fpub $field : $type,)*
    }

    impl QueryStringTrait for $name {
      fn skip(&self) -> Option<i64> { self.skip }
      fn limit(&self) -> Option<i64> { self.limit }
    }
  }
}

pub trait QueryStringTrait {
	fn skip(&self) -> Option<u32>;
	fn limit(&self) -> Option<u32>;
}


pub async fn exec_basic_db_write_endpoint<T: Serialize>(qs: Box<dyn QueryStringTrait>, data: T) -> HttpResponse {
  let skip = qs.skip().unwrap_or(0);
  let limit = qs.limit().unwrap_or(100);

  send_read(
    Arc::clone(&neo4j),
    query,
    db_query_params,
  ).await
  .map(|result| {
    HttpResponse::Ok()
      .json(BaseResponse {
        count: result.0.len(),
        result,
        skip: Some(skip),
        limit: Some(limit),
      })
  })
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}
