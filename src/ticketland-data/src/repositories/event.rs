use diesel::prelude::*;
use diesel::dsl::now;
use diesel_async::RunQueryDsl;
use eyre::Result;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    event::{Event, AccountEvent, EventWithSale},
    sale::Sale,
  },
  schema::{
    events::dsl::{self as events_dsl, *},
    accounts::dsl::{
      self as accounts_dsl,
      accounts,
    },
    account_events::dsl::{
      self as acount_events_dsl,
      account_events,
    },
    sales::dsl::{
      self as sales_dsl,
      sales,
    }
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
      .filter(acount_events_dsl::event_id.eq(id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(acount_events_dsl::account_id)))
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
      .filter(acount_events_dsl::account_id.eq(user_id))
      .inner_join(events.on(event_id.eq(acount_events_dsl::event_id)))
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

  pub async fn read_events(&mut self, skip: u32, limit: u32) -> Result<Vec<Event>> {
    Ok(
      events
      .limit(limit as i64)
      .order_by(events_dsl::created_at.desc())
      .offset((skip * limit) as i64)
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_events_by_category(&mut self, categ: i32, skip: u32, limit: u32) -> Result<Vec<EventWithSale>> {
    let records =  events
    .filter(category.eq(categ))
    .filter(end_date.gt(now))
    .inner_join(sales.on(sales_dsl::event_id.eq(events_dsl::event_id)))
    .order_by(events_dsl::start_date.desc())
    .offset((skip * limit) as i64)
    .load::<(Event, Sale)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }
}
