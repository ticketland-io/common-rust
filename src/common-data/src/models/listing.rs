use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bolt_proto::value::{Value};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SellListing {
  pub account: String,
  pub ticket_nft: String,
  pub ask_price: u64,
}

impl TryFrom<Neo4jResult> for SellListing {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let sell_listing = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut sell_listing = SellListing {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "account" => {
              sell_listing.account = String::try_from(v).expect("cannot convert account");
            },
            "ticket_nft" => {
              sell_listing.ticket_nft = String::try_from(v).expect("cannot convert ticket_nft");
            },
            "ask_price" => {
              sell_listing.ask_price = i64::try_from(v).expect("cannot convert ask_price") as u64;
            },
            _ => panic!("unknown field"),
          }
        }

        sell_listing
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(sell_listing)
  }
}
