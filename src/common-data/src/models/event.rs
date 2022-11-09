use std::{
  collections::HashMap,
  convert::TryFrom,
};
use bolt_proto::value::Value;
use serde::{Deserialize, Serialize};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Event {
  pub event_id: String,
  pub created_at: i64,
  pub event_organizer: String,
  pub event_capacity: String,
  pub file_type: String,
  pub arweave_tx_id: String,
  pub metadata_uploaded: bool,
  pub image_uploaded: bool,
  pub draft: bool,
  pub attended: bool,
  pub name: String,
  pub description: String,
  pub location: String,
  pub venue: String,
  pub event_type: String,
  pub start_date: i64,
  pub end_date: i64,
  pub category: String,
  pub publicity: String,
  pub payment_type: String,
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
            "event_id" => {
              event.event_id = String::try_from(v).expect("cannot convert title");
            },
            "created_at" => {
              event.created_at = i64::try_from(v).expect("cannot convert created_at");
            },
            "event_organizer" => {
              event.event_organizer = String::try_from(v).expect("cannot convert event organizer");
            },
            "event_capacity" => {
              event.event_capacity = String::try_from(v).expect("cannot convert event capacity");
            },
            "name" => {
              event.name = String::try_from(v).expect("cannot convert event name");
            },
            "location" => {
              event.location = String::try_from(v).expect("cannot convert event location");
            },
            "venue" => {
              event.venue = String::try_from(v).expect("cannot convert event venue");
            },
            "event_type" => {
              event.event_type = String::try_from(v).expect("cannot convert event event_type");
            },
            "category" => {
              event.category = String::try_from(v).expect("cannot convert event category");
            },
            "start_date" => {
              event.start_date = i64::try_from(v).expect("cannot convert event start_date");
            },
            "end_date" => {
              event.end_date = i64::try_from(v).expect("cannot convert event end_date");
            },
            "description" => {
              event.description = String::try_from(v).expect("cannot convert event description");
            },
            "file_type" => {
              event.file_type = String::try_from(v).expect("cannot convert file_type");
            },
            "arweave_tx_id" => {
              event.arweave_tx_id = String::try_from(v).expect("cannot convert arweave_tx_id");
            },
            "metadata_uploaded" => {
              event.metadata_uploaded = bool::try_from(v).expect("cannot convert metadata_uploaded");
            },
            "image_uploaded" => {
              event.image_uploaded = bool::try_from(v).expect("cannot convert image_uploaded");
            },
            "draft" => {
              event.draft = bool::try_from(v).expect("cannot convert draft");
            },
            "attended" => {
              event.attended = bool::try_from(v).expect("cannot convert attended");
            },
            "payment_type" => {
              event.payment_type = String::try_from(v).expect("cannot convert payment_type");
            },
            "publicity" => {
              event.publicity = String::try_from(v).expect("cannot convert publicity");
            },
            _ => panic!("unknown field {:?}", k.as_str()),
          }
        }

        event
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(event)
  }
}
