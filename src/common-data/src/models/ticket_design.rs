use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct TicketDesign {
  pub design_id: String,
  pub url: String,
  pub name: String,
  pub file_type: String,
  pub created_at: i64,
}

impl TryFrom<Neo4jResult> for TicketDesign {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let account = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut account = TicketDesign {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "design_id" => {
              account.design_id = String::try_from(v).expect("cannot convert design_id");
            },
            "url" => {
              account.url = String::try_from(v).expect("cannot convert url");
            },
            "name" => {
              account.name = String::try_from(v).expect("cannot convert name");
            },
            "file_type" => {
              account.file_type = String::try_from(v).expect("cannot convert file_type");
            },
            "created_at" => {
              account.created_at = i64::try_from(v).expect("cannot convert created_at");
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
