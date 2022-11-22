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

use super::seat_range::SeatRange;

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

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SaleWithSeatRange {
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
  pub seat_range: SeatRange,
}


impl From<Sale> for SaleWithSeatRange {
  fn from(sale: Sale) -> Self {
    SaleWithSeatRange {
      account: sale.account,
      event_id: sale.event_id,
      created_at: sale.created_at,
      ticket_type_index: sale.ticket_type_index,
      ticket_type_name: sale.ticket_type_name,
      n_tickets: sale.n_tickets,
      sale_start_ts: sale.sale_start_ts,
      sale_end_ts: sale.sale_end_ts,
      sale_type: sale.sale_type,
      seat_range: SeatRange::default()
    }
  }

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
