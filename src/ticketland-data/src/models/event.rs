use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::events;
use super::sale::Sale;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = events)]
pub struct Event {
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<String>,
  pub venue: Option<String>,
  pub event_type: i32,
  pub start_date: Option<NaiveDateTime>,
  pub end_date: Option<NaiveDateTime>,
  pub category: i32,
  pub event_capacity: String,
  pub file_type: Option<String>,
  pub arweave_tx_id: Option<String>,
  pub metadata_uploaded: bool,
  pub image_uploaded: bool,
  pub draft: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct EventWithSale {
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<String>,
  pub venue: Option<String>,
  pub event_type: i32,
  pub start_date: Option<NaiveDateTime>,
  pub end_date: Option<NaiveDateTime>,
  pub category: i32,
  pub event_capacity: String,
  pub file_type: Option<String>,
  pub arweave_tx_id: Option<String>,
  pub metadata_uploaded: bool,
  pub image_uploaded: bool,
  pub draft: bool,
  pub sales: Vec<Sale>,
}

impl EventWithSale {
  pub fn from_tuple(values: Vec<(Event, Sale)>) -> Vec<EventWithSale> {
    values
    .into_iter()
    .fold(HashMap::new(), |mut acc, (event, sale)| {
      let key = event.event_id.clone();

      if acc.contains_key(&key) {
        let mut event: EventWithSale = acc.remove(&key).unwrap();
        event.sales.push(sale);
        acc.insert(key, event);
      } else {
        acc.insert(key, EventWithSale {
          event_id: event.event_id.clone(),
          created_at: event.created_at,
          name: event.name.clone(),
          description: event.description.clone(),
          location: event.location.clone(),
          venue: event.venue.clone(),
          event_type: event.event_type,
          start_date: event.start_date,
          end_date: event.end_date,
          category: event.category,
          event_capacity: event.event_capacity.clone(),
          file_type: event.file_type.clone(),
          arweave_tx_id: event.arweave_tx_id.clone(),
          metadata_uploaded: event.metadata_uploaded,
          image_uploaded: event.image_uploaded,
          draft: event.draft,
          sales: vec![],
        });
      }

      acc
    })
    .into_values()
    .collect()
  }
}
