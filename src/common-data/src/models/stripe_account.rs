use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct StripeAccount {
  pub stripe_uid: String,
  pub account_link: String,
  pub status: i64,
}

impl TryFrom<Neo4jResult> for StripeAccount {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let account = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut account = StripeAccount {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "stripe_uid" => {
              account.stripe_uid = String::try_from(v).expect("cannot convert stripe_uid");
            },
            "account_link" => {
              account.account_link = String::try_from(v).expect("cannot convert account_link");
            },
            "status" => {
              account.status = i64::try_from(v).expect("cannot convert status");
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
