use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::ticket_onchain_accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = ticket_onchain_accounts)]
pub struct TicketOnchainAccount {
  pub ticket_nft: String,
  pub ticket_metadata: String,
}
