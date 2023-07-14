use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::{nft_details, event_nft_details, ticket_type_nft_details};

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = nft_details)]
pub struct NftDetail {
  pub nft_name: String,
  pub nft_description: String,
  pub content_type: String,
  pub arweave_tx_id: String,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = nft_details)]
pub struct NewNftDetail {
  pub nft_name: String,
  pub nft_description: String,
  pub content_type: String,
  pub arweave_tx_id: String,
}

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = event_nft_details)]
pub struct EventNftDetail {
  pub ref_name: String,
  pub event_id: String,
  pub nft_details_id: String,
  #[diesel(embed)]
  pub nft_details: NftDetail,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = event_nft_details)]
pub struct NewEventNftDetail {
  pub ref_name: String,
  pub event_id: String,
  pub nft_details_id: String,
}

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = ticket_type_nft_details)]
pub struct TicketTypeNftDetail {
  pub ref_name: String,
  pub event_id: String,
  pub ticket_type_index: i16,
  pub nft_details_id: String,
  #[diesel(embed)]
  pub nft_details: NftDetail,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = ticket_type_nft_details)]
pub struct NewTicketTypeNftDetail {
  pub ref_name: String,
  pub event_id: String,
  pub ticket_type_index: i16,
  pub nft_details_id: String,
}
