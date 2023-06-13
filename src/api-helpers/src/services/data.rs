use serde::{Deserialize};
use crate::QueryString;
use std::fmt::{
	Display,
	Debug,
	Formatter,
	Result
};
use actix_web::{
  HttpResponse,
  ResponseError,
};

pub trait QueryStringTrait {
  fn skip(&self) -> Option<i64>;
  fn limit(&self) -> Option<i64>;
}

QueryString! {
  pub struct QueryString {}
}

pub struct CustomError {
  pub message: String,
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.message)
  }
}

impl ResponseError for CustomError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::InternalServerError().json(self.message.clone())
  }
}
