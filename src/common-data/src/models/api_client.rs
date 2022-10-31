use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct ApiClient {
  pub client_id: String,
  pub client_secret: String,
}

impl TryFrom<Neo4jResult> for ApiClient {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let client = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut client = ApiClient {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "client_id" => {
              client.client_id = String::try_from(v).expect("cannot convert client_id");
            },
            "client_secret" => {
              client.client_secret = String::try_from(v).expect("cannot convert client_secret");
            },
            _ => panic!("unknown field"),
          }
        }

        client
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(client)
  }
}
