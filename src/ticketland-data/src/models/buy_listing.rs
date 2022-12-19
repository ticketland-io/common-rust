use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::buy_listings;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = buy_listings)]
pub struct BuyListing {
  pub sol_account: String,
  pub account_id: String,
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub bid_price: i64,
  pub is_open: bool,
  pub n_listing: i64,
  pub draft: bool,
  pub closed_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = buy_listings)]
pub struct NewBuyListing<'a> {
  pub account_id: &'a str,
  pub event_id: &'a str,
  pub sol_account: &'a str,
  pub bid_price: i64,
  pub n_listing: i64,
  pub is_open: bool,
  pub draft: bool,
}
