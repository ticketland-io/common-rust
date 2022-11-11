use diesel::prelude::*;
use diesel::result::Error;
use eyre::Result;
use futures::FutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    buy_listing::{NewBuyListing, BuyListing},
  },
  schema::{
    buy_listings::dsl::*,
    tickets::dsl::{
      self as tickets_dsl,
      tickets,
    }
  },
};

impl PostgresConnection {
  pub async fn create_buy_listing<'a>(&mut self, buy_listing: NewBuyListing<'a>) -> Result<()> {
    diesel::insert_into(buy_listings)
    .values(&buy_listing)
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
      .limit(limit)
      .offset(skip * limit)
      .order_by(created_at.desc())
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn cancel_buy_listing(&mut self, account: String) -> Result<()> {
    diesel::update(buy_listings)
    .filter(sol_account.eq(account))
    .set(is_open.eq(true))
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
    .transaction::<_, Error, _>(|conn| async move {
      diesel::update(buy_listings)
      .filter(sol_account.eq(buy_listing_account))
      .set(is_open.eq(false))
      .execute(conn)
      .await?;
      
      diesel::update(tickets)
      .filter(tickets_dsl::ticket_nft.eq(ticket_nft_account))
      .set(tickets_dsl::account_id.eq(new_owner))
      .execute(conn)
      .await?;

      Ok(())
    }.boxed())
    .await?;

    Ok(())
  }
}
