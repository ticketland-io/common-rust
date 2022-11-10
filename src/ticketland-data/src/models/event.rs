use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::{events, account_events};


#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = account_events)]
pub struct AccountEvent {
  pub event_id: String,
  pub account_id: String,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = events)]
pub struct Event {
  pub event_id: String,
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
  pub name: String,
  pub description: String,
  pub location: Option<String>,
  pub venue: Option<String>,
  pub event_type: i32,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i32,
  pub event_capacity: String,
  pub file_type: Option<String>,
  pub arweave_tx_id: Option<String>,
  pub metadata_uploaded: bool,
  pub image_uploaded: bool
}
