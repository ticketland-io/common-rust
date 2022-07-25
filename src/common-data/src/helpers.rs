use actix::prelude::*;
use std::sync::{Arc};
use bolt_client::{Params};
use common::{
  error::{Error, map_bolt_result_err},
  async_helpers::with_retry,
  actor::neo4j::{Neo4jActor, Read, Write},
};
use crate::types::Neo4jResult;

pub async fn send_read(
  neo4j: Arc<Addr<Neo4jActor>>,
  query: &'static str,
  params: Option<Params>,
) -> Result<Neo4jResult, Error> {
  let action = || {
    let params = params.clone();
   
    neo4j.send(Read {
      query: String::from(query),
      params,
      metadata: None
    })
  };

  Ok(
    with_retry(None, None, action)
      .await
      .map_err(|error| Into::<Error>::into(error))
      .map(map_bolt_result_err)??
      .into()
  )
}

pub async fn send_write(
  neo4j: Arc<Addr<Neo4jActor>>,
  query: &'static str,
  params: Option<Params>,
) -> Result<Neo4jResult, Error> {
  let action = || {
    let params = params.clone();
    neo4j.send(Write {
      query: String::from(query),
      params,
      metadata: None
    })
  };

  Ok(
    with_retry(None, None, action)
      .await
      .map_err(|error| Into::<Error>::into(error))
      .map(map_bolt_result_err)??
      .into()
  )
}
