use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    sell_listing::SellListing,
  },
  schema::sell_listings::dsl::*,
};

impl PostgresConnection {
  pub async fn create_sell_listing(&mut self, sell_listing: SellListing) -> Result<()> {
    diesel::insert_into(sell_listings)
    .values(&sell_listing)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_sell_listing(&mut self, account: String) -> Result<SellListing> {
    Ok(
      sell_listings
      .filter(sol_account.eq(account))
      .first(self.borrow_mut())
      .await?
    )
  }
}
