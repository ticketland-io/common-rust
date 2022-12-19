use diesel::prelude::*;
use diesel::result::Error;
use eyre::Result;
use futures::FutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    sell_listing::{NewSellListing, SellListing},
  },
  schema::{
    sell_listings::dsl::*,
    tickets::dsl::{
      self as tickets_dsl,
      tickets,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_sell_listing(&mut self, sell_listing: NewSellListing<'_>) -> Result<()> {
    diesel::insert_into(sell_listings)
    .values(&sell_listing)
    .on_conflict(sol_account)
    .do_update()
    .set(&sell_listing)
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

  pub async fn read_sell_listings_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<SellListing>> {
    Ok(
      sell_listings
      .filter(event_id.eq(evt_id))
      .filter(is_open.eq(true))
      .limit(limit)
      .offset(skip * limit)
      .order_by(created_at.desc())
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn cancel_sell_listing(&mut self, uid: String, account: String) -> Result<()> {
    diesel::update(sell_listings)
    .filter(sol_account.eq(account))
    .filter(account_id.eq(uid))
    .set(is_open.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn fill_sell_listing(
    &mut self,
    sell_listing_account: String,
    ticket_nft_account: String,
    new_owner: String,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| async move {
      diesel::update(sell_listings)
      .filter(sol_account.eq(sell_listing_account))
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

  pub async fn update_sell_listing_draft(&mut self, account: &String, listing_account: &String) -> Result<()> {
    diesel::update(sell_listings)
    .filter(sol_account.eq(listing_account))
    .filter(account_id.eq(account))
    .set(draft.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
}
