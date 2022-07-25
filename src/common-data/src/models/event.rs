use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct Event {
  // TODO: Add additional fields
  pub title: String,
  pub description: String,
}

impl TryFrom<Neo4jResult> for Event {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let account = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut account = Event {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "title" => {
              account.title = String::try_from(v).expect("cannot convert value to staking_token");
            },
            "description" => {
              account.description = String::try_from(v).expect("cannot convert value to staking_token");
            },
            _ => panic!("unknown field"),
          }
        }

        account
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(account)
  }
}
