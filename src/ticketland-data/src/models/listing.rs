use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::listings;

#[derive(Queryable, QueryableByName, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = listings)]
pub struct Listing {
  pub listing_id: String,
  pub listing_sui_address: Option<String>,
  pub account_id: String,
  pub event_id: String,
  pub cnt_sui_address: String,
  pub created_at: Option<NaiveDateTime>,
  pub ask_price: i64,
  pub is_open: bool,
  pub closed_at: Option<NaiveDateTime>,
  pub draft: bool,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = listings)]
pub struct NewListing<'a> {
  pub listing_id: &'a str,
  pub account_id: &'a str,
  pub event_id: &'a str,
  pub listing_sui_address: Option<&'a str>,
  pub cnt_sui_address: &'a str,
  pub ask_price: i64,
  pub is_open: bool,
  pub draft: bool,
}
