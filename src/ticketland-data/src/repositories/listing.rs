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
    listing::{NewListing, Listing},
  },
  schema::{
    listings::dsl::*,
    cnts::dsl::{
      self as cnts_dsl,
      cnts,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_listing(&mut self, listing: NewListing<'_>) -> Result<()> {
    diesel::insert_into(listings)
    .values(&listing)
    .on_conflict(listing_id)
    .do_update()
    .set(&listing)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_listing(&mut self, id: String) -> Result<Listing> {
    Ok(
      listings
      .filter(listing_id.eq(id))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_listings_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<Listing>> {
    Ok(
      listings
      .filter(event_id.eq(evt_id))
      .filter(is_open.eq(true))
      .filter(draft.eq(false))
      .limit(limit)
      .offset(skip * limit)
      .order_by(created_at.desc())
      .load(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_listings_for_account(
    &mut self,
    evt_id: Option<String>,
    uid: String,
    skip: i64,
    limit: i64
  ) -> Result<Vec<Listing>> {
    let evt_id_filter = if let Some(evt_id) = evt_id {
      format!("AND event_id = '{}'", evt_id)
    } else {
      "".to_string()
    };

    let query = sql_query(format!(
      "
      SELECT * FROM listings
      WHERE account_id = '{}' AND is_open = true {}
      ORDER BY created_at DESC
      LIMIT {} OFFSET {}
      ",
      uid,
      evt_id_filter,
      limit,
      skip * limit
    ));

    Ok(query.load::<Listing>(self.borrow_mut()).await?)
  }

  pub async fn cancel_listing(&mut self, uid: String, id: String) -> Result<()> {
    diesel::update(listings)
    .filter(listing_id.eq(id))
    .filter(account_id.eq(uid))
    .set(is_open.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn fill_listing(
    &mut self,
    id: String,
    cnt_address: String,
    new_owner: String,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::update(listings)
      .filter(listing_id.eq(id))
      .set((closed_at.eq(dsl::now), is_open.eq(false)))
      .execute(conn)
      .await?;

      diesel::update(cnts)
      .filter(cnts_dsl::cnt_sui_address.eq(cnt_address))
      .set(cnts_dsl::account_id.eq(new_owner))
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
  }

  pub async fn update_listing_draft(&mut self, account: &String, id: &String) -> Result<()> {
    diesel::update(listings)
    .filter(listing_id.eq(id))
    .filter(account_id.eq(account))
    .set(draft.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
}
