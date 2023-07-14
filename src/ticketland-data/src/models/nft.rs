use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::{event_nfts, ticket_type_nfts};

#[derive(Queryable, QueryableByName, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = ticket_type_nfts)]
pub struct TicketTypeNft {
  pub ticket_type_nft_sui_address: String,
  pub cnt_sui_address: String,
  pub account_id: String,
  pub ref_name: String,
  pub event_id: String,
  pub ticket_type_index: i16,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[diesel(table_name = ticket_type_nfts)]
pub struct NewTicketTypeNft {
  pub ticket_type_nft_sui_address: String,
  pub cnt_sui_address: String,
  pub account_id: String,
  pub ref_name: String,
  pub event_id: String,
  pub ticket_type_index: i16,
}

#[derive(Queryable, QueryableByName, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = event_nfts)]
pub struct EventNft {
  pub event_nft_sui_address: Option<String>,
  pub account_id: String,
  pub event_id: String,
  pub ref_name: String,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = event_nfts)]
pub struct NewEventNft {
  pub event_nft_sui_address: Option<String>,
  pub account_id: String,
  pub ref_name: String,
  pub event_id: String,
}
