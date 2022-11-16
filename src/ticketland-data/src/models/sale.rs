use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use diesel::{
  prelude::*,
  FromSqlRow,
  AsExpression,
};
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::{
    serialize as to_milli_ts,
    deserialize as from_milli_ts,
  }
};
use diesel_as_jsonb::AsJsonb;
use crate::schema::sales;

#[derive(AsJsonb)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SaleType {
  Free {},
  FixedPrice {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    price: u64
  },
  Refundable {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    price: u64
  },
  DutchAuction {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    start_price: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
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

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = sales)]
pub struct Sale {
  pub account: String,
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub ticket_type_name: String,
  pub n_tickets: i32,
  #[serde(serialize_with = "to_milli_ts")]
  pub sale_start_ts: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub sale_end_ts: NaiveDateTime,
  pub sale_type: SaleType,
}


#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = sales)]
pub struct NewSale {
  pub account: String,
  pub event_id: String,
  pub ticket_type_index: i16,
  pub ticket_type_name: String,
  pub n_tickets: i32,
  #[serde(deserialize_with = "from_milli_ts")]
  pub sale_start_ts: NaiveDateTime,
  #[serde(deserialize_with = "from_milli_ts")]
  pub sale_end_ts: NaiveDateTime,
  pub sale_type: SaleType,
}
