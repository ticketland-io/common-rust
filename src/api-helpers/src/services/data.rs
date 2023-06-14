use serde::{Deserialize};
use crate::QueryString;
use std::fmt::Debug;

pub trait QueryStringTrait {
  fn skip(&self) -> Option<i64>;
  fn limit(&self) -> Option<i64>;
}

QueryString! {
  pub struct QueryString {}
}
