use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    ticket::Ticket,
    ticket_onchain_account::TicketOnchainAccount,
  },
  schema::{
    tickets::dsl::*,
    ticket_onchain_accounts::dsl::{
      self as ticket_onchain_accounts_dsl,
      ticket_onchain_accounts,
    }
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

  pub async fn read_user_tickets_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<Ticket>> {
    Ok(
      tickets
      .filter(event_id.eq(evt_id))
      .order_by(created_at.desc())
      .limit(limit)
      .offset(skip * limit)
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

  pub async fn read_ticket_by_ticket_metadata(&mut self, ticket_metadata: String) -> Result<Ticket> {
    Ok(
      ticket_onchain_accounts
      .filter(ticket_onchain_accounts_dsl::ticket_metadata.eq(ticket_metadata))
      .inner_join(tickets.on(ticket_nft.eq(ticket_onchain_accounts_dsl::ticket_nft)))
      .first::<(TicketOnchainAccount, Ticket)>(self.borrow_mut())
      .await?
      .1
    )
  }
}
