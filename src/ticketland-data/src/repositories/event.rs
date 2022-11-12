use diesel::prelude::*;
use diesel::dsl::now;
use diesel_async::RunQueryDsl;
use eyre::Result;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    event::{Event, EventWithSale},
    sale::Sale,
  },
  schema::{
    events::dsl::{
      self as events_dsl,
      events,
    },
    accounts::dsl::{
      self as accounts_dsl,
      accounts,
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
    .on_conflict(events_dsl::event_id)
    .do_update()
    .set(&event)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }

  pub async fn update_metadata_uploaded(&mut self, id: String, arweave_tx: String) -> Result<()> {
    diesel::update(events)
    .filter(events_dsl::event_id.eq(id))
    .set(events_dsl::arweave_tx_id.eq(arweave_tx))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn update_image_uploaded(&mut self, id: String) -> Result<()> {
    diesel::update(events)
    .filter(events_dsl::event_id.eq(id))
    .set(events_dsl::image_uploaded.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_event_organizer_account(&mut self, evt_id: String) -> Result<Account> {
    Ok(
      events
      .filter(events_dsl::event_id.eq(evt_id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(events_dsl::account_id)))
      .select(accounts::all_columns())
      .first::<Account>(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_account_events(&mut self, user_id: String) -> Result<Vec<Event>> {
    Ok(
      accounts
      .filter(accounts_dsl::uid.eq(user_id))
      .inner_join(events.on(events_dsl::account_id.eq(accounts_dsl::uid)))
      .select(events::all_columns())
      .load::<Event>(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_event(&mut self, evt_id: String) -> Result<Event> {
    Ok(
      events
      .filter(events_dsl::event_id.eq(evt_id))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_event_and_organizer(&mut self, evt_id: String) -> Result<(Event, String)> {
    Ok(
      events
      .filter(events_dsl::event_id.eq(evt_id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(events_dsl::account_id)))
      .select((events::all_columns(), accounts_dsl::pubkey))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_events(&mut self, skip: i64, limit: i64) -> Result<Vec<Event>> {
    Ok(
      events
      .limit(limit)
      .offset(skip * limit)
      .order_by(events_dsl::created_at.desc())
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_events_by_category(&mut self, categ: i16, skip: i64, limit: i64) -> Result<Vec<EventWithSale>> {
    let records =  events
    .filter(events_dsl::category.eq(categ))
    .filter(events_dsl::end_date.gt(now))
    .inner_join(sales.on(sales_dsl::event_id.eq(events_dsl::event_id)))
    .limit(limit)
    .offset(skip * limit)
    .order_by(events_dsl::start_date.desc())
    .load::<(Event, Sale)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn update_draft(&mut self, evt_id: String) -> Result<()> {
    diesel::update(events)
    .filter(events_dsl::event_id.eq(evt_id))
    .set(events_dsl::draft.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_event_with_sales(&mut self, evt_id: String) -> Result<Vec<EventWithSale>> {
    let records =  events
    .filter(events_dsl::event_id.eq(evt_id))
    .inner_join(sales.on(sales_dsl::event_id.eq(events_dsl::event_id)))
    .load::<(Event, Sale)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }
}
