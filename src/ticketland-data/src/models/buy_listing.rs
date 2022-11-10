use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::buy_listings;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = buy_listings)]
pub struct BuyListing {
  pub id: i32,
  pub account_id: String,
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub sol_account: String,
  pub bid_price: i64,
  pub is_open: bool,
}
