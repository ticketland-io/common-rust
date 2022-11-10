use serde::{Deserialize, Serialize};
use diesel::{
  prelude::*,
  FromSqlRow,
  AsExpression,
};
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use diesel_as_jsonb::AsJsonb;
use crate::schema::sales;

#[derive(AsJsonb)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SaleType {
  Free {},
  FixedPrice {
    price: u64
  },
  Refundable {
    price: u64
  },
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}


impl Default for SaleType {
  fn default() -> Self {
    Self::Free {}
  }
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = sales)]
pub struct Sale {
  pub id: String,
  pub event_id: String,
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
  pub account: String,
  pub ticket_type_index: i16,
  pub ticket_type_name: String,
  pub n_tickets: i32,
  #[serde(serialize_with = "to_milli_ts")]
  pub sale_start_ts: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub sale_end_ts: NaiveDateTime,
  pub sale_type: SaleType,
}
