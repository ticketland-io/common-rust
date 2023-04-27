use diesel::{
  prelude::*,
  result::Error,
  sql_query,
};
use chrono::NaiveDateTime;
use diesel_async::{AsyncConnection, RunQueryDsl};
use eyre::Result;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    event::{Event, EventWithSale, TicketImage, TicketImageUpdate},
    sale::Sale,
    seat_range::SeatRange,
    event::AttendedTicketCount
  },
  schema::{
    events::dsl::{
      self as events_dsl,
      events,
    },
    ticket_images::dsl::{
      self as ticket_images_dsl,
      ticket_images,
    },
    accounts::dsl::{
      self as accounts_dsl,
      accounts,
    },
  },
};

impl PostgresConnection {
  pub async fn upsert_event(&mut self, event: Event, ticket_images_list: Vec<TicketImage>) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::insert_into(events)
      .values(&event)
      .on_conflict(events_dsl::event_id)
      .do_update()
      .set(&event)
      .execute(conn)
      .await?;

      diesel::insert_into(ticket_images)
      .values(&ticket_images_list)
      .on_conflict((ticket_images_dsl::event_id, ticket_images_dsl::ticket_image_type))
      .do_nothing()
      .execute(conn)
      .await?;

      Ok(())
    }))
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

  pub async fn read_account_events(
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
      filters.push(format!("events.start_date <= '{0}'", start_date_to));
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
        WHERE events.account_id = '{0}' AND events.draft = false AND {1}
        LIMIT {2}
        OFFSET {3}
      ) events
      INNER JOIN ticket_images
      ON ticket_images.event_id = events.event_id
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

    let records = query.load::<(Event, TicketImage, Sale, SeatRange)>(self.borrow_mut()).await?;

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
        SELECT * FROM (SELECT * FROM events WHERE events.draft = false) events
        LIMIT {}
        OFFSET {}
      ) events
      INNER JOIN ticket_images
      ON ticket_images.event_id = events.event_id
      INNER JOIN sales
      ON sales.event_id = events.event_id
      INNER JOIN seat_ranges
      ON seat_ranges.sale_account = sales.account
      ORDER BY events.start_date
      ", limit, skip * limit
    ));

    let records = query.load::<(Event, TicketImage, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn read_filtered_events(
    &mut self,
    category: Option<i16>,
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
      filters.push(format!("events.start_date >= '{0}'", start_date_from));
    };

    if let Some(start_date_to) = start_date_to {
      filters.push(format!("events.start_date <= '{0}'", start_date_to));
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
          ((sale_type->'FixedPrice'->'price')::numeric <= {0}
          OR (sale_type->'Free') = '{{}}')
        ", price_range.1));
      }
    };

    let filters_query = if filters.len() > 0 {
      filters.join(" AND ")
    } else {
      "true = true".to_string()
    };

    let query = sql_query(format!("
      WITH filtered_events AS (
        SELECT *
        FROM (SELECT * FROM events WHERE events.draft = false) events
        INNER JOIN ticket_images using(event_id)
        INNER JOIN sales using(event_id)
        INNER JOIN seat_ranges ON seat_ranges.sale_account = sales.account
        WHERE {}
      )
      SELECT * FROM filtered_events
      INNER JOIN (
        SELECT DISTINCT event_id FROM filtered_events
        ORDER BY event_id
        LIMIT {} OFFSET {}
      ) limited_events ON limited_events.event_id = filtered_events.event_id
    ", filters_query, limit, skip * limit)
    );

    let records = query
    .load::<(Event, TicketImage, Sale, SeatRange)>(self.borrow_mut())
    .await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn update_event_draft(&mut self, evt_id: String) -> Result<()> {
    diesel::update(events)
    .filter(events_dsl::event_id.eq(evt_id))
    .set(events_dsl::draft.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_event_with_sales(&mut self, evt_id: String, draft: bool) -> Result<Vec<EventWithSale>> {
    let query = sql_query(format!(
      "
      SELECT *
      FROM (
        SELECT * FROM events
        WHERE events.event_id = '{}' AND events.draft = {}
      ) events
      INNER JOIN ticket_images
      ON ticket_images.event_id = events.event_id
      INNER JOIN sales
      ON sales.event_id = events.event_id
      INNER JOIN seat_ranges
      ON seat_ranges.sale_account = sales.account
      ", evt_id, draft
    ));

    let records = query.load::<(Event, TicketImage, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn read_attended_tickets_count(&mut self, evt_id: String) -> Result<Vec<AttendedTicketCount>> {
    let query = sql_query(format!(
      "
      SELECT sales.ticket_type_index,
      COUNT(tickets.ticket_type_index) AS total_count,
      COUNT(CASE WHEN attended = TRUE THEN 1 END) AS attended_count
      FROM sales 
      LEFT OUTER JOIN tickets ON sales.ticket_type_index = tickets.ticket_type_index AND sales.event_id = tickets.event_id
      WHERE sales.event_id = '{}'
      GROUP BY sales.ticket_type_index;
      ", evt_id
    ));

    Ok(query.load::<AttendedTicketCount>(self.borrow_mut()).await?)
  }

  pub async fn read_account_ticket_events(
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
      filters.push(format!("events.start_date <= '{0}'", start_date_to));
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
        SELECT events.* FROM events
        WHERE events.draft = false AND
              EXISTS (SELECT * FROM tickets WHERE events.event_id = tickets.event_id AND tickets.account_id = '{}' AND {})
        LIMIT {}
        OFFSET {}
      ) events
      INNER JOIN ticket_images ON ticket_images.event_id = events.event_id
      INNER JOIN sales ON events.event_id = sales.event_id
      INNER JOIN seat_ranges ON seat_ranges.sale_account = sales.account
      ORDER BY events.start_date
      ",
      user_id,
      filters_query,
      limit,
      skip * limit,
    ));

    let records = query.load::<(Event, TicketImage, Sale, SeatRange)>(self.borrow_mut()).await?;

    Ok(EventWithSale::from_tuple(records))
  }

  pub async fn update_ticket_images_uploaded(
    &mut self,
    ticket_image_updates: Vec<TicketImageUpdate>,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      for ticket_image_update in ticket_image_updates {
        diesel::update(ticket_images)
        .filter(ticket_images_dsl::event_id.eq(ticket_image_update.event_id.clone()))
        .set((ticket_images_dsl::uploaded.eq(true), ticket_images_dsl::arweave_tx_id.eq(ticket_image_update.arweave_tx_id.clone())))
        .execute(conn)
        .await?;
      };

      Ok(())
    }))
    .await?;

    Ok(())
  }

}
