use serde::{Deserialize, Serialize};
use diesel::{prelude::*, sql_types};
use chrono::{
  NaiveDateTime,
};
use crate::schema::tickets;
use super::{ticket_onchain_account::TicketOnchainAccount, event::Event, sale::Sale};

#[derive( Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = tickets)]
pub struct Ticket {
  pub ticket_nft: String,
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
pub struct PartialSellListing {
  #[diesel(sql_type = sql_types::VarChar)]
  sol_account: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TicketWithMetadata {
  pub ticket_nft: String,
  pub ticket_metadata: String,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
  pub draft: bool,
  pub sell_listing: Option<PartialSellListing>,
}

impl TicketWithMetadata {
  pub fn from_tuple(values: Vec<(Ticket, TicketOnchainAccount, Option<PartialSellListing>)>) -> Vec<Self> {
    values
    .into_iter()
    .map(|(ticket, ticket_onchain_account, sell_listing)| {
      TicketWithMetadata {
        ticket_nft: ticket.ticket_nft,
        ticket_metadata: ticket_onchain_account.ticket_metadata,
        event_id: ticket.event_id,
        account_id: ticket.account_id,
        created_at: ticket.created_at,
        ticket_type_index: ticket.ticket_type_index,
        seat_name: ticket.seat_name,
        seat_index: ticket.seat_index,
        attended: ticket.attended,
        draft: ticket.draft,
        sell_listing
      }
    })
    .collect()
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TicketWithEvent {
  pub ticket_nft: String,
  pub ticket_metadata: String,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
  pub draft: bool,
  pub sell_listing: Option<PartialSellListing>,
  pub event: Event,
  pub sale: Sale,
}

impl TicketWithEvent {
  pub fn from_tuple(values: Vec<(Ticket, TicketOnchainAccount, Event, Sale, Option<PartialSellListing>)>) -> Vec<Self> {
    values
    .into_iter()
    .map(|(ticket, ticket_onchain_account, event, sale, sell_listing)| {
      TicketWithEvent {
        ticket_nft: ticket.ticket_nft,
        ticket_metadata: ticket_onchain_account.ticket_metadata,
        event_id: ticket.event_id,
        account_id: ticket.account_id,
        created_at: ticket.created_at,
        ticket_type_index: ticket.ticket_type_index,
        seat_name: ticket.seat_name,
        seat_index: ticket.seat_index,
        attended: ticket.attended,
        draft: ticket.draft,
        sell_listing,
        event,
        sale,
      }
    })
    .collect()
  }
}
