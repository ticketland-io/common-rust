use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::offers;

#[derive(Queryable, QueryableByName, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = offers)]
pub struct Offer {
  // TODO: consider this being an option
  pub offer_id: String,
  pub offer_sui_address: Option<String>,
  pub account_id: String,
  pub event_id: String,
  pub ticket_type_index: i16,
  pub created_at: Option<NaiveDateTime>,
  pub bid_price: i64,
  pub is_open: bool,
  pub closed_at: Option<NaiveDateTime>,
  pub draft: bool,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = offers)]
pub struct NewOffer<'a> {
  pub offer_id: &'a str,
  pub offer_sui_address: Option<&'a str>,
  pub account_id: &'a str,
  pub event_id: &'a str,
  pub ticket_type_index: i16,
  pub bid_price: i64,
  pub is_open: bool,
  pub draft: bool,
}
