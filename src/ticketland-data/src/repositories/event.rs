use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use eyre::Result;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    event::{Event, AccountEvent},
  },
  schema::{
    events::dsl::*,
    accounts::dsl::*,
    account_events::dsl::{
      account_events,
      event_id as account_event_event_id,
      account_id as account_events_account_id,
    },
  },
};

impl PostgresConnection {
  pub async fn upsert_event(&mut self, event: Event) -> Result<()> {
    diesel::insert_into(events)
    .values(&event)
    .on_conflict(event_id)
    .do_update()
    .set(&event)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }

  pub async fn update_metadata_uploaded(&mut self, id: String, arweave_tx: String) -> Result<()> {
    diesel::update(events)
    .filter(event_id.eq(id))
    .set(arweave_tx_id.eq(arweave_tx))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn update_image_uploaded(&mut self, id: String) -> Result<()> {
    diesel::update(events)
    .filter(event_id.eq(id))
    .set(image_uploaded.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_event_organizer_account(&mut self, id: String) -> Result<Account> {
    Ok(
      account_events
      .filter(account_event_event_id.eq(id))
      .inner_join(accounts.on(uid.eq(account_events_account_id)))
      .load::<(AccountEvent, Account)>(self.borrow_mut())
      .await?
      .into_iter()
      .map(|r| r.1)
      .collect::<Vec<_>>()
      .remove(0)
    )
  }

  pub async fn read_account_events(&mut self, user_id: String) -> Result<Vec<Event>> {
    Ok(
      account_events
      .filter(account_events_account_id.eq(user_id))
      .inner_join(events.on(event_id.eq(account_event_event_id)))
      .load::<(AccountEvent, Event)>(self.borrow_mut())
      .await?
      .into_iter()
      .map(|r| r.1)
      .collect::<Vec<_>>()
    )
  }

  pub async fn read_event(&mut self, id: String) -> Result<Event> {
    Ok(
      events
      .filter(event_id.eq(id))
      .first(self.borrow_mut())
      .await?
    )
  }
}
