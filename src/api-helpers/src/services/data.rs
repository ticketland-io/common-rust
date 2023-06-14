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
