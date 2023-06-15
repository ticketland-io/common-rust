use diesel::{prelude::*, sql_query};
use eyre::Result;
use diesel::result::Error;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    ticket::{Cnt, CntWithMetadata, PartialSellListing, TicketWithEvent},
    ticket_onchain_account::TicketOnchainAccount, sale::Sale, event::Event,
  },
  schema::{
    cnts::dsl::{self as cnts_dsl},
    ticket_onchain_accounts::dsl::{
      self as ticket_onchain_accounts_dsl,
      ticket_onchain_accounts,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_user_ticket(&mut self, user_ticket: Cnt, ticket: TicketOnchainAccount) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      // ignore if the ticket nft is already stored
      diesel::insert_into(ticket_onchain_accounts)
      .values(&ticket)
      .on_conflict(ticket_onchain_accounts_dsl::cnt_nft)
      .do_nothing()
      .execute(conn)
      .await?;

      diesel::insert_into(cnts_dsl::cnts)
      .values(&user_ticket)
      .on_conflict(cnts_dsl::cnt_nft)
      .do_update()
      .set(&user_ticket)
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
  }

  pub async fn read_tickets_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<CntWithMetadata>> {
    let query = sql_query(format!(
      "
      SELECT DISTINCT tickets.*, sell_listings.sol_account, ticket_onchain_accounts.ticket_metadata
      FROM (
        SELECT * FROM tickets
        WHERE tickets.event_id = '{}'
        LIMIT {}
        OFFSET {}
      ) tickets
      INNER JOIN ticket_onchain_accounts
      ON ticket_onchain_accounts.ticket_nft = tickets.ticket_nft
      LEFT JOIN sell_listings ON (
        sell_listings.ticket_nft = tickets.ticket_nft
        AND sell_listings.is_open = TRUE AND sell_listings.draft = FALSE
      )
      ORDER BY tickets.created_at
      ", evt_id, limit, skip * limit
    ));

    let records = query.load::<(Cnt, TicketOnchainAccount, Option<PartialSellListing>)>(self.borrow_mut()).await?;

    Ok(CntWithMetadata::from_tuple(records))
  }

  pub async fn read_user_tickets(
    &mut self,
    uid: String,
    event_id: Option<String>,
    skip: i64,
    limit: i64,
  ) -> Result<Vec<TicketWithEvent>> {
    let event_id_filter = event_id.map_or("".to_string(), |event_id| {
      format!("AND tickets.event_id = '{}'", event_id)
    });

    let query = sql_query(format!(
      "
      SELECT DISTINCT tickets.*, events.*, sales.*, sell_listings.sol_account, ticket_onchain_accounts.ticket_metadata
      FROM (
        SELECT * FROM tickets
        WHERE tickets.account_id = '{}' {}
        LIMIT {}
        OFFSET {}
      ) tickets
      INNER JOIN ticket_onchain_accounts
      ON ticket_onchain_accounts.ticket_nft = tickets.ticket_nft
      INNER JOIN events ON tickets.event_id = events.event_id
      INNER JOIN sales ON tickets.event_id = sales.event_id AND tickets.ticket_type_index = sales.ticket_type_index
      LEFT JOIN sell_listings ON (
        sell_listings.ticket_nft = tickets.ticket_nft
        AND sell_listings.is_open = TRUE AND sell_listings.draft = FALSE
      )
      ORDER BY tickets.created_at
      ",
      uid,
      event_id_filter,
      limit,
      skip * limit,
    ));

    let records = query.load::<(Cnt, TicketOnchainAccount, Event, Sale, Option<PartialSellListing>)>(self.borrow_mut()).await?;

    Ok(TicketWithEvent::from_tuple(records))
  }

  pub async fn update_attended(&mut self, ticket_nft_acc: String) -> Result<()> {
    diesel::update(cnts_dsl::cnts)
    .filter(cnts_dsl::cnt_nft.eq(ticket_nft_acc))
    .set(cnts_dsl::attended.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn has_attended(&mut self, ticket_nft_acc: String) -> Result<bool> {
    Ok(
      cnts_dsl::cnts
      .filter(cnts_dsl::cnt_nft.eq(ticket_nft_acc))
      .select(cnts_dsl::attended)
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_ticket_by_ticket_metadata(&mut self, ticket_metadata: String) -> Result<Cnt> {
    Ok(
      ticket_onchain_accounts
      .filter(ticket_onchain_accounts_dsl::ticket_metadata.eq(ticket_metadata))
      .inner_join(cnts_dsl::cnts.on(cnts_dsl::cnt_nft.eq(ticket_onchain_accounts_dsl::cnt_nft)))
      .first::<(TicketOnchainAccount, Cnt)>(self.borrow_mut())
      .await?
      .1
    )
  }

  pub async fn read_ticket(&mut self, ticket_nft_acc: String) -> Result<Vec<Cnt>> {
    Ok(
      cnts_dsl::cnts
      .filter(cnts_dsl::cnt_nft.eq(ticket_nft_acc))
      .load::<Cnt>(self.borrow_mut())
      .await?
    )
  }
}
