use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    ticket::Ticket,
  },
  schema::{
    tickets::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn upsert_user_ticket(&mut self, ticket: Ticket) -> Result<()> {
    diesel::insert_into(tickets)
    .values(&ticket)
    .on_conflict(ticket_nft)
    .do_update()
    .set(&ticket)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
}
