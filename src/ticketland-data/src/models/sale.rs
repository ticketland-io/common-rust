use serde::{Deserialize, Serialize};
use diesel::{
  prelude::*,
  FromSqlRow,
  AsExpression,
};
use chrono::NaiveDateTime;
use diesel_as_jsonb::AsJsonb;
use crate::schema::sales;

#[derive(AsJsonb)]
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = sales)]
pub struct Sale {
  pub id: String,
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub account: String,
  pub ticket_type_index: i16,
  pub ticket_type_name: String,
  pub n_tickets: i32,
  pub sale_start_ts: Option<NaiveDateTime>,
  pub sale_end_ts: Option<NaiveDateTime>,
  pub sale_type: SaleType,
}
