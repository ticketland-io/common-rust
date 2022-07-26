use std::sync::Arc;
use actix::prelude::*;
use actix_web::{HttpResponse};
use serde::{Serialize, Deserialize};
use bolt_client::{Params};
use ticketland_core::{
  error::Error,
  actor::{neo4j::Neo4jActor},
};
use common_data::{
  helpers::{send_read, send_write},
  types::{Neo4jResult},
};
use super::http::internal_server_error;

/// Example `impl_query_string!(QueryString);`
#[macro_export]
macro_rules! impl_query_string {
  ($ty:ident) => {
    impl QueryStringTrait for $ty {
      fn skip(&self) -> Option<u32> { self.skip }
      fn limit(&self) -> Option<u32> { self.limit }
    }
  }
}

pub trait QueryStringTrait {
  fn skip(&self) -> Option<u32>;
  fn limit(&self) -> Option<u32>;
}

#[derive(Deserialize, Default)]
pub struct QueryString {
  pub skip: Option<u32>,
  pub limit: Option<u32>
}

impl_query_string!(QueryString);

#[derive(Serialize)]
pub struct BaseResponse {
  pub count: usize,
  pub skip: Option<u32>,
  pub limit: Option<u32>,
  pub result: Neo4jResult,
}

#[derive(Serialize)]
pub struct Neo4jBaseResponse {
  pub result: Neo4jResult,
}

pub type DbQueryBuilder = Box<dyn Fn() -> (&'static str, Option<Params>)>;

pub async fn exec_basic_db_read_endpoint(
  neo4j: Arc<Addr<Neo4jActor>>,
  qs: Box<dyn QueryStringTrait>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let skip = qs.skip().unwrap_or(0);
  let limit = qs.limit().unwrap_or(100);
  let (query, db_query_params) = db_query_builder();

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

pub async fn exec_basic_db_read_endpoint_no_qs(
  neo4j: Arc<Addr<Neo4jActor>>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let (query, db_query_params) = db_query_builder();

  send_read(
    Arc::clone(&neo4j),
    query,
    db_query_params,
  ).await
  .map(|result| {
    HttpResponse::Ok()
      .json(Neo4jBaseResponse {result})
  })
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}

pub async fn exec_basic_db_write_endpoint(
  neo4j: Arc<Addr<Neo4jActor>>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let (query, db_query_params) = db_query_builder();

  send_write(
    Arc::clone(&neo4j),
    query,
    db_query_params,
  ).await
  .map(|_| HttpResponse::Ok().finish())
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}