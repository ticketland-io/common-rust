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

  pub async fn read_user_tickets_for_event(&mut self, evt_id: String, skip: u32, limit: u32) -> Result<Vec<Ticket>> {
    Ok(
      tickets
      .filter(event_id.eq(evt_id))
      .limit(limit as i64)
      .order_by(created_at.desc())
      .offset((skip * limit) as i64)
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn update_attended(&mut self, ticket_nft_acc: String) -> Result<()> {
    diesel::update(tickets)
    .filter(ticket_nft.eq(ticket_nft_acc))
    .set(attended.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn has_attended(&mut self, ticket_nft_acc: String) -> Result<bool> {
    Ok(
      tickets
      .filter(ticket_nft.eq(ticket_nft_acc))
      .select(attended)
      .first(self.borrow_mut())
      .await?
    )
  }
}
