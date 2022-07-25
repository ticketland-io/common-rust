use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use common::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct Account {
  pub uid: String,
  pub mnemonic: String,
}

impl TryFrom<Neo4jResult> for Account {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let account = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut account = Account {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "uid" => {
              account.uid = String::try_from(v).expect("cannot convert value to staking_token");
            },
            "mnemonic" => {
              account.mnemonic = String::try_from(v).expect("cannot convert value to staking_token");
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
