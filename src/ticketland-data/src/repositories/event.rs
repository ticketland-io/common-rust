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
    event::AttendedTicketCount
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

  pub async fn read_account_events (
    &mut self,
    user_id: String,
    start_date_from: Option<NaiveDateTime>, 
    start_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64, 
    limit: i64
) -> Result<Vec<EventWithSale>> {
    let mut filters = vec![];

    if let Some(name) = name {
      filters.push(format!("events.name ILIKE '%{}%'", name));
    };

    if let Some(start_date_from) = start_date_from {
      filters.push(format!("events.start_date >= '{0}'", start_date_from));
    };

    if let Some(start_date_to) = start_date_to {
      filters.push(format!("events.end_date <= '{0}'", start_date_to));
    };

    let filters_query = if filters.len() > 0 {
      filters.join(" AND ")
    } else {
      "true = true".to_string()
    };

    let query = sql_query(format!(
      "
      SELECT *
      FROM (
        SELECT * FROM events
        WHERE events.account_id = '{0}' AND {1}
        limit {2}
        offset {3}
      ) events
      INNER JOIN sales
      ON sales.event_id = events.event_id
      INNER JOIN seat_ranges
      ON seat_ranges.sale_account = sales.account
      ORDER BY events.start_date
      ",
      user_id,
      filters_query,
      limit,
      skip * limit,
    ));

    let records = query.load::<(Event, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
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

  pub async fn read_filtered_events(
    &mut self, category: Option<i16>,
    price_range: Option<(u32, u32)>,
    start_date_from: Option<NaiveDateTime>,
    start_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64,
    limit: i64,
  ) -> Result<Vec<EventWithSale>> {
    let mut filters = vec![];

    if let Some(category) = category {
      filters.push(format!("events.category = {}", category));
    };

    if let Some(name) = name {
      filters.push(format!("events.name ILIKE '%{}%'", name));
    };

    if let Some(start_date_from) = start_date_from {
      filters.push(format!("events.start_date >= '{0}'::date", start_date_from));
    };

    if let Some(start_date_to) = start_date_to {
      filters.push(format!("events.start_date <= '{0}'::date", start_date_to));
    };

    if let Some(price_range) = price_range {
      // There are 2 possible cases for (x, y) with the precondition that x <= y
      // 1. (x > 0, y > 0) -> We include only FixedPrice sale type
      // 2. (x >= 0, y >= 0) -> We include Free and FixedPrice sale types
      if price_range.0 > 0 {
        filters.push(format!("
          (sale_type->'FixedPrice'->'price')::numeric >= {0}
          AND (sale_type->'FixedPrice'->'price')::numeric <= {1}
        ", price_range.0, price_range.1));
      } else {
        filters.push(format!("
          (sale_type->'FixedPrice'->'price')::numeric <= {0}
          OR (sale_type->'Free') = '{{}}'
        ", price_range.1));
      }
    };

    let filters_query = if filters.len() > 0 {
      filters.join(" AND ")
    } else {
      "true = true".to_string()
    };

    let query = sql_query(format!("
      SELECT events.*, sales.*, seat_ranges.*
      FROM (SELECT * FROM events limit {0} offset {1}) events
      INNER JOIN sales ON events.event_id = sales.event_id
      INNER JOIN seat_ranges ON seat_ranges.sale_account = sales.account
      WHERE {2}
      ORDER BY events.event_id
    ", limit, skip * limit, filters_query)
    );

    let records = query
    .load::<(Event, Sale, SeatRange)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn update_event_draft(&mut self, evt_id: String) -> Result<()> {
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

  pub async fn read_attended_tickets_count(&mut self, evt_id: String) -> Result<Vec<AttendedTicketCount>> {
    let query = sql_query(format!(
      "
      SELECT sales.ticket_type_index,
      COUNT (*) AS total_count,
      COUNT(CASE WHEN attended = TRUE THEN 1 END) AS attended_count
      FROM tickets
      INNER JOIN sales ON tickets.ticket_type_index = sales.ticket_type_index AND tickets.event_id=sales.event_id 
      WHERE tickets.event_id = '{}' 
      GROUP BY sales.ticket_type_index;
      ", evt_id
    ));

    Ok(query.load::<AttendedTicketCount>(self.borrow_mut()).await?)
  }

  pub async fn read_account_ticket_events (
    &mut self,
    user_id: String,
    start_date_from: Option<NaiveDateTime>, 
    start_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64,
    limit: i64
) -> Result<Vec<EventWithSale>> {
    let mut filters = vec![];

    if let Some(name) = name {
      filters.push(format!("events.name ILIKE '%{}%'", name));
    };

    if let Some(start_date_from) = start_date_from {
      filters.push(format!("events.start_date >= '{0}'", start_date_from));
    };

    if let Some(start_date_to) = start_date_to {
      filters.push(format!("events.end_date <= '{0}'", start_date_to));
    };

    let filters_query = if filters.len() > 0 {
      filters.join(" AND ")
    } else {
      "true = true".to_string()
    };

    let query = sql_query(format!(
      "
      SELECT *
      FROM events
      INNER JOIN sales on events.event_id = sales.event_id
      INNER JOIN seat_ranges on seat_ranges.sale_account = sales.account
      WHERE events.event_id IN (
        SELECT event_id FROM tickets
      	WHERE tickets.account_id = '{0}' AND {1}
        limit {2}
        offset {3}
      )
      ORDER BY events.start_date
      ",
      user_id,
      filters_query,
      limit,
      skip * limit,
    ));

    let records = query.load::<(Event, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

}
