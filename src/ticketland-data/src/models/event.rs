use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::events;
use super::{sale::{Sale, SaleWithSeatRange}, seat_range::SeatRange};

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = events)]
pub struct Event {
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<String>,
  pub venue: Option<String>,
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_capacity: String,
  pub file_type: Option<String>,
  pub arweave_tx_id: Option<String>,
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
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_capacity: String,
  pub file_type: Option<String>,
  pub arweave_tx_id: Option<String>,
  pub image_uploaded: bool,
  pub draft: bool,
  pub sales: Vec<SaleWithSeatRange>,
}

impl EventWithSale {
  pub fn from_tuple(values: Vec<(Event, Sale, SeatRange)>) -> Vec<EventWithSale> {
    values
    .into_iter()
    .fold(HashMap::new(), |mut acc, (event, sale, seat_range)| {
      let key = event.event_id.clone();
      let mut sale_with_seat_range = SaleWithSeatRange::from(sale);
      sale_with_seat_range.seat_range = seat_range;

      if acc.contains_key(&key) {
        let mut event: EventWithSale = acc.remove(&key).unwrap();
        // TODO: handle case where a Seat maps with multiple SeatRange.
        event.sales.push(sale_with_seat_range);
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
          visibility: event.visibility,
          start_date: event.start_date,
          end_date: event.end_date,
          category: event.category,
          event_capacity: event.event_capacity.clone(),
          file_type: event.file_type.clone(),
          arweave_tx_id: event.arweave_tx_id.clone(),
          image_uploaded: event.image_uploaded,
          draft: event.draft,
          sales: vec![sale_with_seat_range],
        });
      }

      acc
    })
    .into_values()
    .collect()
  }
}
