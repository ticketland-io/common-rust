use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use diesel::{prelude::*, sql_types};
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::{events, ticket_images};
use super::{sale::{Sale, SaleWithSeatRange}, seat_range::SeatRange};
use diesel::{
  FromSqlRow,
  AsExpression,
};
use diesel_as_jsonb::AsJsonb;

#[derive(AsJsonb)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub name: String,
     #[serde(deserialize_with = "deserialize_number_from_string")]
    pub latitude: f64,
     #[serde(deserialize_with = "deserialize_number_from_string")]
    pub longitude: f64,
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
  pub venue: Option<String>,
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_capacity: String,
  pub arweave_tx_id: Option<String>,
  pub webbundle_arweave_tx_id: Option<String>,
  pub draft: bool,
}

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = ticket_images)]
pub struct TicketImage {
  pub event_id: String,
  pub ticket_type_index: i16,
  pub ticket_nft_index: i16,
  pub name: String,
  pub description: String,
  pub content_type: String,
  pub arweave_tx_id: Option<String>,
  pub uploaded: bool,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = ticket_images)]
pub struct TicketImageUpdate {
  pub event_id: String,
  pub ticket_type_index: i16,
  pub ticket_nft_index: i16,
  pub name: String,
  pub description: String,
  pub arweave_tx_id: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct EventWithSale {
  pub event_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub name: String,
  pub description: String,
  pub location: Option<Location>,
  pub venue: Option<String>,
  pub event_type: i16,
  pub visibility: i16,
  #[serde(serialize_with = "to_milli_ts")]
  pub start_date: NaiveDateTime,
  #[serde(serialize_with = "to_milli_ts")]
  pub end_date: NaiveDateTime,
  pub category: i16,
  pub event_capacity: String,
  pub arweave_tx_id: Option<String>,
  pub webbundle_arweave_tx_id: Option<String>,
  pub draft: bool,
  pub sales: Vec<SaleWithSeatRange>,
  pub ticket_images: Vec<TicketImage>,
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


impl EventWithSale {
  pub fn from_tuple(values: Vec<(Event, TicketImage, Sale, SeatRange)>) -> Vec<EventWithSale> {
    values
    .into_iter()
    .fold(Vec::new(), |mut acc: Vec<EventWithSale>, (event, ticket_image, sale, seat_range)| {
      let key = event.event_id.clone();
      // TODO: handle case where a Seat maps with multiple SeatRange.
      let mut sale_with_seat_range = SaleWithSeatRange::from(sale);
      sale_with_seat_range.seat_range = seat_range;

      let existing_index = acc
      .iter()
      .position(|item| item.event_id == key);


      if let Some(index) = existing_index {
        let existing_ticket_type_index = acc[index].ticket_images
        .iter()
        .position(|item| item.ticket_type_index == ticket_image.ticket_type_index && item.ticket_nft_index == ticket_image.ticket_nft_index);

        if existing_ticket_type_index == None {
          acc[index].ticket_images.push(ticket_image);
        }

        let existing_sale_index = acc[index].sales
        .iter()
        .position(|item| item.account == sale_with_seat_range.account);
        if existing_sale_index == None {
          acc[index].sales.push(sale_with_seat_range);
        }
      } else {
        acc.push(EventWithSale {
          event_id: event.event_id.clone(),
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
          event_capacity: event.event_capacity.clone(),
          arweave_tx_id: event.arweave_tx_id.clone(),
          webbundle_arweave_tx_id: event.webbundle_arweave_tx_id,
          draft: event.draft,
          sales: vec![sale_with_seat_range],
          ticket_images: vec![ticket_image],
        });
      }

      acc
    })
  }
}
