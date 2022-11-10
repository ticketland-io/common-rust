use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
pub struct Ticket {
  pub ticket_nft: String,
  pub ticket_metadata: String,
  pub ticket_type_index: u8,
  pub seat_index: u32,
  pub seat_name: String,
  pub created_at: i64,
  pub attended: bool,
}

impl TryFrom<Neo4jResult> for Ticket {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let ticket = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut ticket = Ticket {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "ticket_nft" => {
              ticket.ticket_nft = String::try_from(v).expect("cannot convert ticket_nft");
            },
            "ticket_metadata" => {
              ticket.ticket_metadata = String::try_from(v).expect("cannot convert ticket_metadata");
            },
            "ticket_type_index" => {
              ticket.ticket_type_index = i8::try_from(v).expect("cannot convert ticket_type_index") as u8;
            },
            "seat_index" => {
              ticket.seat_index = i32::try_from(v).expect("cannot convert seat_index") as u32;
            },
            "seat_name" => {
              ticket.seat_name = String::try_from(v).expect("cannot convert seat_name");
            },
            "created_at" => {
              ticket.created_at = i64::try_from(v).expect("cannot convert created_at");
            },
            "attended" => {
              ticket.attended = bool::try_from(v).expect("cannot convert attended");
            },
            _ => panic!("unknown field"),
          }
        }

        ticket
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(ticket)
  }
}
