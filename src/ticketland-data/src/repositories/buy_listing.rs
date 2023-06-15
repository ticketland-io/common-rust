use diesel::{
  prelude::*,
  result::Error,
  dsl,
  sql_query,
};
use eyre::Result;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    buy_listing::{NewBuyListing, BuyListing},
  },
  schema::{
    buy_listings::dsl::*,
    cnts::dsl::{
      self as tickets_dsl,
      cnts,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_buy_listing(&mut self, buy_listing: NewBuyListing<'_>) -> Result<()> {
    diesel::insert_into(buy_listings)
    .values(&buy_listing)
    .on_conflict(sol_account)
    .do_update()
    .set(&buy_listing)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_buy_listing(&mut self, account: String) -> Result<BuyListing> {
    Ok(
      buy_listings
      .filter(sol_account.eq(account))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_buy_listings_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<BuyListing>> {
    Ok(
      buy_listings
      .filter(event_id.eq(evt_id))
      .filter(is_open.eq(true))
      .limit(limit)
      .offset(skip * limit)
      .order_by(created_at.desc())
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_buy_listings_for_account(
    &mut self,
    evt_id: Option<String>,
    uid: String,
    skip: i64,
    limit: i64
  ) -> Result<Vec<BuyListing>> {
    let evt_id_filter = if let Some(evt_id) = evt_id {
      format!("AND event_id = '{}'", evt_id)
    } else {
      "".to_string()
    };

    let query = sql_query(format!(
      "
      SELECT * FROM buy_listings
      WHERE account_id = '{}' AND is_open = true {}
      ORDER BY created_at DESC
      LIMIT {} OFFSET {}
      ",
      uid,
      evt_id_filter,
      limit,
      skip * limit
    ));

    Ok(query.load::<BuyListing>(self.borrow_mut()).await?)
  }

  pub async fn cancel_buy_listing(&mut self, uid: String, account: String) -> Result<()> {
    diesel::update(buy_listings)
    .filter(sol_account.eq(account))
    .filter(account_id.eq(uid))
    .set(is_open.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn fill_buy_listing(
    &mut self,
    buy_listing_account: String,
    ticket_nft_account: String,
    new_owner: String,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::update(buy_listings)
      .filter(sol_account.eq(buy_listing_account))
      .set((closed_at.eq(dsl::now), is_open.eq(false)))
      .execute(conn)
      .await?;

      diesel::update(cnts)
      .filter(tickets_dsl::cnt_nft.eq(ticket_nft_account))
      .set(tickets_dsl::account_id.eq(new_owner))
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
  }

  pub async fn update_buy_listing_draft(&mut self, account: &String, listing_account: &String) -> Result<()> {
    diesel::update(buy_listings)
    .filter(sol_account.eq(listing_account))
    .filter(account_id.eq(account))
    .set(draft.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
}
