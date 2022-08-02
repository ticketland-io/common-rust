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
  pub event_id: String,
  pub image_type: String,
  pub metadata_cid: String,
  pub metadata_uploaded: bool,
  pub image_uploaded: bool,
}

impl TryFrom<Neo4jResult> for Event {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let event = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut event = Event {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "title" => {
              event.event_id = String::try_from(v).expect("cannot convert title");
            },
            "image_type" => {
              event.image_type = String::try_from(v).expect("cannot convert image_type");
            },
            "metadata_cid" => {
              event.metadata_cid = String::try_from(v).expect("cannot convert metadata_cid");
            },
            "metadata_uploaded" => {
              event.metadata_uploaded = bool::try_from(v).expect("cannot convert metadata_uploaded");
            },
            "image_uploaded" => {
              event.image_uploaded = bool::try_from(v).expect("cannot convert image_uploaded");
            },
            _ => panic!("unknown field"),
          }
        }

        event
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(event)
  }
}
