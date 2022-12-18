use diesel::{
  prelude::*,
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
    seat_range::SeatRange,
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
    },
    seat_ranges::dsl:: {
      self as seat_ranges_dsl,
      seat_ranges
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

  pub async fn update_webbundle_uploaded(&mut self, id: String, arweave_tx: String) -> Result<()> {
    diesel::update(events)
    .filter(events_dsl::event_id.eq(id))
    .set(events_dsl::webbundle_arweave_tx_id.eq(arweave_tx))
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
        limit {}
        offset {}
      ) events
      INNER JOIN sales
      ON sales.event_id = events.event_id
      INNER JOIN seat_ranges
      ON seat_ranges.sale_account = sales.account
      ORDER BY events.start_date
      ", limit, skip * limit
    ));

    let records = query.load::<(Event, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn read_filtered_events(&mut self, category: Option<i16>, price_range: [u32; 2], start_date: Option<NaiveDateTime>, end_date: Option<NaiveDateTime>, name: Option<String>, skip: i64, limit: i64) -> Result<Vec<EventWithSale>> {
    let mut query = events.inner_join(sales.on(sales_dsl::event_id.eq(events_dsl::event_id)))
    .inner_join(seat_ranges.on(seat_ranges_dsl::sale_account.eq(sales_dsl::account)))
    .into_boxed();

    if let Some(category) = category {
      query = query.filter(events_dsl::category.eq(category));
    }

    if let Some(name) = name {
      query = query.filter(events_dsl::name.ilike(format!("%{}%", name.clone())));
    }

    if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
      query = query.filter(events_dsl::start_date.between(start_date, end_date));
    }

    if let (Some(start_date), None) = (start_date, end_date) {
      query = query.filter(events_dsl::start_date.gt(start_date));
    }

    if let (None, Some(end_date)) = (start_date, end_date) {
      query = query.filter(events_dsl::end_date.lt(end_date));
    }

    query = query.limit(limit).offset(skip);

    // TODO: fix order_by query
    query = query.order_by(events_dsl::event_id.asc()); 
    // TODO: add price range and skip limit

    let records = query.load::<(Event, Sale, SeatRange)>(self.borrow_mut()).await?;

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
    .inner_join(seat_ranges.on(seat_ranges_dsl::sale_account.eq(sales_dsl::account)))
    .load::<(Event, Sale, SeatRange)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }
}
