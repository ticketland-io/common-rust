use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    buy_listing::BuyListing,
  },
  schema::buy_listings::dsl::*,
};

impl PostgresConnection {
  pub async fn create_buy_listing(&mut self, buy_listing: BuyListing) -> Result<()> {
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

  pub async fn read_buy_listings_for_event(&mut self, evt_id: String, skip: u32, limit: u32) -> Result<Vec<BuyListing>> {
    Ok(
      buy_listings
      .filter(event_id.eq(evt_id))
      .limit(limit as i64)
      .order_by(created_at.desc())
      .offset((skip * limit) as i64)
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
}
