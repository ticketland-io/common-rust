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
use crate::schema::ticket_types;
use super::{
  seat_range::SeatRange,
  nft_detail::TicketTypeNftDetail,
};


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
#[diesel(table_name = ticket_types)]
pub struct TicketType {
  pub ticket_type_sui_address: Option<String>,
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
pub struct ExtendedTicketType {
  pub ticket_type_sui_address: Option<String>,
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
  pub ticket_type_nft_details: Vec<TicketTypeNftDetail>,
}


impl From<TicketType> for ExtendedTicketType {
  fn from(ticket_type: TicketType) -> Self {
    ExtendedTicketType {
      ticket_type_sui_address: ticket_type.ticket_type_sui_address,
      event_id: ticket_type.event_id,
      created_at: ticket_type.created_at,
      ticket_type_index: ticket_type.ticket_type_index,
      ticket_type_name: ticket_type.ticket_type_name,
      n_tickets: ticket_type.n_tickets,
      sale_start_ts: ticket_type.sale_start_ts,
      sale_end_ts: ticket_type.sale_end_ts,
      sale_type: ticket_type.sale_type,
      seat_range: SeatRange::default(),
      ticket_type_nft_details: vec![],
    }
  }
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = ticket_types)]
pub struct NewTicketType {
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
