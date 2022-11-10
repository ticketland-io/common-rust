use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::sell_listings;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = sell_listings)]
pub struct SellListing {
  pub id: i32,
  pub account_id: String,
  pub ticket_nft: String,
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub sol_account: String,
  pub ask_price: i64,
  pub is_open: bool,
}
