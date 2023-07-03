use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_query};
use diesel::result::Error;
use diesel_async::{AsyncConnection, RunQueryDsl};
use eyre::Result;
use crate::models::nft_detail::TicketTypeNftDetail;
use crate::models::ticket_type::TicketType;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    event::{Event, ExtendedEvent},
    ticket_type::{NewTicketType},
    seat_range::SeatRange,
    event::AttendedTicketCount,
    nft_detail::{NewNftDetail, NewEventNftDetail, NewTicketTypeNftDetail},
    // properties::NewProperty
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
    ticket_types::dsl::{
      self as ticket_types_dsl,
      ticket_types,
    },
    seat_ranges::dsl::{
      self as seat_ranges_dsl,
      seat_ranges,
    },
    nft_details::dsl::{
      self as nft_details_dsl,
      nft_details,
    },
    event_nft_details::dsl::{
      self as event_nft_details_dsl,
      event_nft_details,
    },
    ticket_type_nft_details::dsl::{
      self as ticket_type_nft_details_dsl,
      ticket_type_nft_details,
    },
    // properties::dsl::{
    //   self as properties_dsl,
    //   properties,
    // },
  },
};

impl PostgresConnection {
  pub async fn upsert_event(
    &mut self,
    event: Event,
    seat_ranges_list: Vec<SeatRange>,
    ticket_types_list: Vec<NewTicketType>,
    nft_details_list: Vec<NewNftDetail>,
    event_nft_detail: NewEventNftDetail,
    ticket_type_nfts_details_list: Vec<NewTicketTypeNftDetail>,
    // properties_list: Vec<NewProperty>,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::insert_into(events)
      .values(&event)
      .on_conflict(events_dsl::event_id)
      .do_update()
      .set(&event)
      .execute(conn)
      .await?;

      diesel::insert_into(ticket_types)
      .values(&ticket_types_list)
      .on_conflict((ticket_types_dsl::event_id, ticket_types_dsl::ticket_type_index))
      .do_nothing()
      .execute(conn)
      .await?;

      diesel::insert_into(seat_ranges)
      .values(&seat_ranges_list)
      .on_conflict((
        seat_ranges_dsl::event_id,
        seat_ranges_dsl::ticket_type_index,
        seat_ranges_dsl::l,
        seat_ranges_dsl::r
      ))
      .do_nothing()
      .execute(conn)
      .await?;

      diesel::insert_into(nft_details)
      .values(&nft_details_list)
      .on_conflict(nft_details_dsl::arweave_tx_id)
      .do_nothing()
      .execute(conn)
      .await?;

      diesel::insert_into(event_nft_details)
      .values(&event_nft_detail)
      .on_conflict(event_nft_details_dsl::ref_name)
      .do_nothing()
      .execute(conn)
      .await?;

      diesel::insert_into(ticket_type_nft_details)
      .values(&ticket_type_nfts_details_list)
      .on_conflict(ticket_type_nft_details_dsl::ref_name)
      .do_nothing()
      .execute(conn)
      .await?;

      Ok(())
    }))
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
    end_date_from: Option<NaiveDateTime>,
    end_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64,
    limit: i64
  ) -> Result<Vec<ExtendedEvent>> {
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

    if let Some(end_date_from) = end_date_from {
      filters.push(format!("events.end_date >= '{0}'", end_date_from));
    };

    if let Some(end_date_to) = end_date_to {
      filters.push(format!("events.end_date <= '{0}'", end_date_to));
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
      INNER JOIN ticket_types USING(event_id)
      INNER JOIN seat_ranges USING(event_id, ticket_type_index)
      INNER JOIN ticket_type_nft_details USING(event_id, ticket_type_index)
      INNER JOIN nft_details
      ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
      ORDER BY events.start_date
      ",
      user_id,
      filters_query,
      limit,
      skip * limit,
    ));

    let records = query
    .load::<(Event, TicketType, SeatRange, TicketTypeNftDetail)>(self.borrow_mut())
    .await?;

    Ok(ExtendedEvent::from_tuple(records))
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

  pub async fn read_filtered_events(
    &mut self,
    category: Option<i16>,
    price_range: Option<(u32, u32)>,
    start_date_from: Option<NaiveDateTime>,
    start_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64,
    limit: i64,
  ) -> Result<Vec<ExtendedEvent>> {
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
        INNER JOIN ticket_types USING(event_id)
        INNER JOIN seat_ranges USING(event_id, ticket_type_index)
        INNER JOIN ticket_type_nft_details USING(event_id, ticket_type_index)
        INNER JOIN nft_details
        ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
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
    .load::<(Event, TicketType, SeatRange, TicketTypeNftDetail)>(self.borrow_mut())
    .await?;

    Ok(ExtendedEvent::from_tuple(records))
  }

  pub async fn commit_event(
    &mut self,
    evt_id: String,
    event_sui_address: String,
    organizer_cap: String,
    operator_cap: String,
    event_nft: String,
    event_capacity_bitmap_address: String,
    ticket_type_accounts: Vec<String>,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      let update_event_query = sql_query(format!(
        "
        UPDATE events
        SET event_sui_address = '{1}',
            organizer_cap = '{2}',
            operator_cap = '{3}',
            event_nft = '{4}',
            event_capacity_bitmap_address = '{5}',
            draft = false
        WHERE events.event_id = '{0}';
        ",
        evt_id,
        event_sui_address,
        organizer_cap,
        operator_cap,
        event_nft,
        event_capacity_bitmap_address,
      ));

      update_event_query.execute(conn).await?;

      // We cannot do multiple UPDATEs in one query, so these need to be separate
      for (ticket_type_index, ticket_type_account) in ticket_type_accounts.iter().enumerate() {
        let update_query = format!(
          "
          UPDATE ticket_types
          SET ticket_type_sui_address = '{}'
          WHERE ticket_types.event_id = '{}' AND ticket_type_index = {};
          ",
          ticket_type_account,
          evt_id.clone(),
          ticket_type_index
        );

        sql_query(update_query).execute(conn).await?;
      }

      Ok(())
    }))
    .await?;


    Ok(())
  }

  pub async fn read_event_with_ticket_types(&mut self, evt_id: String, draft: bool) -> Result<Vec<ExtendedEvent>> {
    let query = sql_query(format!(
      "
      SELECT * FROM (
        SELECT * FROM events
        WHERE events.event_id = '{}' AND events.draft = {}
      ) events
      INNER JOIN ticket_types
      ON ticket_types.event_id = events.event_id
      INNER JOIN seat_ranges
      ON seat_ranges.event_id = ticket_types.event_id AND seat_ranges.ticket_type_index = ticket_types.ticket_type_index
      INNER JOIN ticket_type_nft_details
      ON ticket_type_nft_details.event_id = ticket_types.event_id AND ticket_type_nft_details.ticket_type_index = ticket_types.ticket_type_index
      INNER JOIN nft_details
      ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
      ", evt_id, draft
    ));

    let records = query
    .load::<(Event, TicketType, SeatRange, TicketTypeNftDetail)>(self.borrow_mut())
    .await?;

    Ok(ExtendedEvent::from_tuple(records))
  }

  pub async fn read_attended_tickets_count(&mut self, evt_id: String) -> Result<Vec<AttendedTicketCount>> {
    let query = sql_query(format!(
      "
      SELECT ticket_types.ticket_type_index,
      COUNT(cnts.ticket_type_index) AS total_count,
      COUNT(CASE WHEN attended = TRUE THEN 1 END) AS attended_count
      FROM ticket_types
      LEFT OUTER JOIN cnts ON ticket_types.ticket_type_index = cnts.ticket_type_index AND ticket_types.event_id = cnts.event_id
      WHERE ticket_types.event_id = '{}'
      GROUP BY ticket_types.ticket_type_index;
      ", evt_id
    ));

    Ok(query.load::<AttendedTicketCount>(self.borrow_mut()).await?)
  }

  pub async fn read_account_ticket_events(
    &mut self,
    user_id: String,
    start_date_from: Option<NaiveDateTime>,
    start_date_to: Option<NaiveDateTime>,
    end_date_from: Option<NaiveDateTime>,
    end_date_to: Option<NaiveDateTime>,
    name: Option<String>,
    skip: i64,
    limit: i64
  ) -> Result<Vec<ExtendedEvent>> {
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

    if let Some(end_date_from) = end_date_from {
      filters.push(format!("events.end_date >= '{0}'", end_date_from));
    };

    if let Some(end_date_to) = end_date_to {
      filters.push(format!("events.end_date <= '{0}'", end_date_to));
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
              EXISTS (SELECT * FROM cnts WHERE events.event_id = cnts.event_id AND cnts.account_id = '{}' AND {})
        LIMIT {}
        OFFSET {}
      ) events
      INNER JOIN ticket_types USING(event_id)
      INNER JOIN seat_ranges USING(event_id, ticket_type_index)
      INNER JOIN ticket_type_nft_details USING(event_id, ticket_type_index)
      INNER JOIN nft_details
      ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
      ORDER BY events.start_date
      ",
      user_id,
      filters_query,
      limit,
      skip * limit,
    ));

    let records = query
    .load::<(Event, TicketType, SeatRange, TicketTypeNftDetail)>(self.borrow_mut())
    .await?;

    Ok(ExtendedEvent::from_tuple(records))
  }
}
