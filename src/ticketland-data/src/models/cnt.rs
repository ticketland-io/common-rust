use serde::{Deserialize, Serialize};
use diesel::{prelude::*, sql_types};
use chrono::{
  NaiveDateTime,
};
use crate::schema::cnts;
use super::{
  event::Event,
  ticket_type::{ExtendedTicketType, TicketType},
  seat_range::SeatRange,
  nft_detail::TicketTypeNftDetail,
  nft::TicketTypeNft
};

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = cnts)]
pub struct CNT {
  pub cnt_sui_address: Option<String>,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
  pub draft: bool,
}

#[derive(QueryableByName, Serialize, Deserialize, Clone, Default)]
pub struct PartialListing {
  #[diesel(sql_type = sql_types::VarChar)]
  listing_sui_address: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CNTWithMetadata {
  pub cnt_sui_address: Option<String>,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
  pub draft: bool,
  pub listing: Option<PartialListing>,
  pub ticket_type: ExtendedTicketType,
  pub nfts: Vec<TicketTypeNft>,
}

impl CNTWithMetadata {
  pub fn from_tuple(values: Vec<(CNT, TicketType, SeatRange, TicketTypeNftDetail, Option<TicketTypeNft>, Option<PartialListing>)>) -> Vec<Self> {
    values
    .into_iter()
    .fold(
      Vec::new(),
      |mut acc: Vec<CNTWithMetadata>, (cnt, ticket_type, seat_range, ticket_type_nft_details, ticket_type_nft, listing)| {
      let key = cnt.cnt_sui_address.clone();
        let mut extended_ticket_type = ExtendedTicketType::from(ticket_type);
        extended_ticket_type.seat_range = seat_range;
        extended_ticket_type.ticket_type_nft_details = vec![ticket_type_nft_details.clone()];

        let existing_cnt_index = acc
        .iter()
        .position(|item| item.cnt_sui_address == key);

        if let Some(index) = existing_cnt_index {
          acc[index].ticket_type.ticket_type_nft_details.push(ticket_type_nft_details);

          if let Some(ticket_type_nft) = ticket_type_nft {
            acc[index].nfts.push(ticket_type_nft);
          }
        } else {
          acc.push(CNTWithMetadata {
            cnt_sui_address: cnt.cnt_sui_address,
            event_id: cnt.event_id,
            account_id: cnt.account_id,
            created_at: cnt.created_at,
            ticket_type_index: cnt.ticket_type_index,
            seat_name: cnt.seat_name,
            seat_index: cnt.seat_index,
            attended: cnt.attended,
            draft: cnt.draft,
            listing,
            ticket_type: extended_ticket_type,
            nfts: ticket_type_nft.map_or(vec![], |ticket_type_nft| vec![ticket_type_nft]),
          });
        }

        acc
    })
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CNTWithEvent {
  pub cnt_sui_address: Option<String>,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
  pub draft: bool,
  pub listing: Option<PartialListing>,
  pub event: Event,
  pub ticket_type: ExtendedTicketType,
  pub nfts: Vec<TicketTypeNft>,
}

impl CNTWithEvent {
  pub fn from_tuple(values: Vec<(CNT, Event, TicketType, SeatRange, TicketTypeNftDetail, Option<TicketTypeNft>, Option<PartialListing>)>) -> Vec<Self> {
    values
    .into_iter()
    .fold(
      Vec::new(),
      |mut acc: Vec<CNTWithEvent>, (cnt, event, ticket_type, seat_range, ticket_type_nft_details, ticket_type_nft, listing)| {
      let key = cnt.cnt_sui_address.clone();
        let mut extended_ticket_type = ExtendedTicketType::from(ticket_type);
        extended_ticket_type.seat_range = seat_range;
        extended_ticket_type.ticket_type_nft_details = vec![ticket_type_nft_details.clone()];

        let existing_cnt_index = acc
        .iter()
        .position(|item| item.cnt_sui_address == key);

        if let Some(index) = existing_cnt_index {
          acc[index].ticket_type.ticket_type_nft_details.push(ticket_type_nft_details);

          if let Some(ticket_type_nft) = ticket_type_nft {
            acc[index].nfts.push(ticket_type_nft);
          }
        } else {
          acc.push(CNTWithEvent {
            cnt_sui_address: cnt.cnt_sui_address,
            event_id: cnt.event_id,
            account_id: cnt.account_id,
            created_at: cnt.created_at,
            ticket_type_index: cnt.ticket_type_index,
            seat_name: cnt.seat_name,
            seat_index: cnt.seat_index,
            attended: cnt.attended,
            draft: cnt.draft,
            listing,
            event,
            ticket_type: extended_ticket_type,
            nfts: ticket_type_nft.map_or(vec![], |ticket_type_nft| vec![ticket_type_nft]),
          });
        }

        acc
    })
  }
}
