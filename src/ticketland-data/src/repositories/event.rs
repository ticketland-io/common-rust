use diesel::prelude::*;
use diesel::{
  sql_query,
};
use chrono::NaiveDateTime;
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

  pub async fn read_events(&mut self, skip: i64, limit: i64) -> Result<Vec<EventWithSale>> {
    let query = sql_query(format!(
      "
      SELECT *
      FROM (
        SELECT * FROM events 
        WHERE events.start_date > NOW()
        limit {} 
        offset {}
      ) events
      INNER JOIN sales 
      ON sales.event_id = events.event_id
      ORDER BY events.start_date
      ", limit, skip * limit
    ));

    let records = query.load::<(Event, Sale)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn read_filtered_events(&mut self, categ: i16, priceRange: [u32; 2], date: NaiveDateTime, name: String, skip: i64, limit: i64) -> Result<Vec<EventWithSale>> {
    let query = sql_query(format!(
      // TODO: add priceRange filtering
      "
      SELECT *
      FROM (
        SELECT * FROM events 
        WHERE events.category = {} AND events.start_date > NOW() AND events.start_date > {} AND events.name LIKE AND events.description LIKE {}
        limit {} 
        offset {}
      ) events
      INNER JOIN sales 
      ON sales.event_id = events.event_id
      ORDER BY events.start_date
      ", categ, date, name, limit, skip * limit
    ));

    let records = query.load::<(Event, Sale)>(self.borrow_mut()).await?;

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
