use serde::{Deserialize, Serialize};
// use serde_aux::prelude::*;
use diesel::{prelude::*, sql_types};
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::events;
use super::{
  ticket_type::{TicketType, ExtendedTicketType},
  seat_range::SeatRange,
  nft_detail::TicketTypeNftDetail,
};
use diesel::{
  FromSqlRow,
  AsExpression,
};
use diesel_as_jsonb::AsJsonb;

#[derive(AsJsonb)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub name: String,
    // #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub latitude: f32,
    // #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub longitude: f32,
}

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = events)]
pub struct Event {
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<Location>,
  pub venue: String,
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_sui_address: Option<String>,
  pub organizer_cap: Option<String>,
  pub operator_cap: Option<String>,
  pub event_nft: Option<String>,
  pub event_capacity_bitmap_address: Option<String>,
  pub webbundle_arweave_tx_id: Option<String>,
  pub draft: bool,
}

impl Event {
  pub fn is_default(&self) -> bool {
    return self.name == ""
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ExtendedEvent {
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<Location>,
  pub venue: String,
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_sui_address: Option<String>,
  pub organizer_cap: Option<String>,
  pub operator_cap: Option<String>,
  pub event_nft: Option<String>,
  pub event_capacity_bitmap_address: Option<String>,
  pub webbundle_arweave_tx_id: Option<String>,
  pub draft: bool,
  pub ticket_types: Vec<ExtendedTicketType>,
}

#[derive(QueryableByName, Serialize)]
pub struct AttendedTicketCount {
  #[diesel(sql_type = sql_types::SmallInt)]
  ticket_type_index: i16,
  #[diesel(sql_type = sql_types::BigInt)]
  total_count: i64,
  #[diesel(sql_type = sql_types::BigInt)]
  attended_count: i64,
}

fn append_nft_details_to_ticket_type(ticket_types: &mut Vec<ExtendedTicketType>, nft_detail: TicketTypeNftDetail) -> &Vec<ExtendedTicketType> {

  let ticket_type_index = ticket_types
  .iter()
  .position(|item| item.ticket_type_index == nft_detail.ticket_type_index);

  // If the nft_details does not refer to this ticket_type, return
  if let Some(ticket_type_index) = ticket_type_index {
    let nft_details_index = ticket_types[ticket_type_index].ticket_type_nft_details
    .iter()
    .position(|item| item.nft_details_id == nft_detail.nft_details_id);

    if None == nft_details_index {
      ticket_types[ticket_type_index].ticket_type_nft_details.push(nft_detail);
    }
  }

  ticket_types
}

impl ExtendedEvent {
  pub fn from_tuple(values: Vec<(Event, TicketType, SeatRange, TicketTypeNftDetail)>) -> Vec<ExtendedEvent> {
    values
    .into_iter()
    .fold(
      Vec::new(),
     |mut acc: Vec<ExtendedEvent>, (event, ticket_type, seat_range, ticket_type_nft_details)| {

      let key = event.event_id.clone();
      // TODO: handle case where a Seat maps with multiple SeatRange.
      let mut extended_ticket_type = ExtendedTicketType::from(ticket_type);
      extended_ticket_type.seat_range = seat_range;
      extended_ticket_type.ticket_type_nft_details = vec![ticket_type_nft_details.clone()];

      let existing_event_index = acc
      .iter()
      .position(|item| item.event_id == key);

      if let Some(index) = existing_event_index {
        let existing_ticket_type_index = acc[index].ticket_types
        .iter()
        .position(|item| item.ticket_type_sui_address == extended_ticket_type.ticket_type_sui_address);


        if None == existing_ticket_type_index {
          acc[index].ticket_types.push(extended_ticket_type);
        } else {
          append_nft_details_to_ticket_type(&mut acc[index].ticket_types, ticket_type_nft_details);
        }
      } else {
        acc.push(ExtendedEvent {
          event_id: event.event_id.clone(),
          account_id: event.account_id.clone(),
          created_at: event.created_at,
          name: event.name.clone(),
          description: event.description.clone(),
          location: event.location.clone(),
          venue: event.venue.clone(),
          event_type: event.event_type,
          visibility: event.visibility,
          start_date: event.start_date,
          end_date: event.end_date,
          category: event.category,
          event_sui_address: event.event_sui_address.clone(),
          organizer_cap: event.organizer_cap.clone(),
          operator_cap: event.operator_cap.clone(),
          event_nft: event.event_nft.clone(),
          event_capacity_bitmap_address: event.event_capacity_bitmap_address.clone(),
          webbundle_arweave_tx_id: event.webbundle_arweave_tx_id,
          draft: event.draft,
          ticket_types: vec![extended_ticket_type],
        });
      }

      acc
    })
  }
}
