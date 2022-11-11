use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    ticket_onchain_account::TicketOnchainAccount,
  },
  schema::{
    ticket_onchain_accounts::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn create_ticket_nft(&mut self, ticket: TicketOnchainAccount) -> Result<()> {
    diesel::insert_into(ticket_onchain_accounts)
    .values(&ticket)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
}
