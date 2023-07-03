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
    offer::{NewOffer, Offer},
  },
  schema::{
    offers::dsl::*,
    cnts::dsl::{
      self as cnts_dsl,
      cnts,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_offer(&mut self, offer: NewOffer<'_>) -> Result<()> {
    diesel::insert_into(offers)
    .values(&offer)
    .on_conflict(offer_id)
    .do_update()
    .set(&offer)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_offer(&mut self, id: String) -> Result<Offer> {
    Ok(
      offers
      .filter(offer_id.eq(id))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_offers_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<Offer>> {
    Ok(
      offers
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

  pub async fn read_offers_for_account(
    &mut self,
    evt_id: Option<String>,
    uid: String,
    skip: i64,
    limit: i64
  ) -> Result<Vec<Offer>> {
    let evt_id_filter = if let Some(evt_id) = evt_id {
      format!("AND event_id = '{}'", evt_id)
    } else {
      "".to_string()
    };

    let query = sql_query(format!(
      "
      SELECT * FROM offers
      WHERE account_id = '{}' AND is_open = true {}
      ORDER BY created_at DESC
      LIMIT {} OFFSET {}
      ",
      uid,
      evt_id_filter,
      limit,
      skip * limit
    ));

    Ok(query.load::<Offer>(self.borrow_mut()).await?)
  }

  pub async fn cancel_offer(&mut self, uid: String, id: String) -> Result<()> {
    diesel::update(offers)
    .filter(offer_id.eq(id))
    .filter(account_id.eq(uid))
    .set(is_open.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn fill_offer(
    &mut self,
    id: String,
    cnt_sui_address: String,
    new_owner: String,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::update(offers)
      .filter(offer_id.eq(id))
      .set((closed_at.eq(dsl::now), is_open.eq(false)))
      .execute(conn)
      .await?;

      diesel::update(cnts)
      .filter(cnts_dsl::cnt_sui_address.eq(cnt_sui_address))
      .set(cnts_dsl::account_id.eq(new_owner))
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
  }

  pub async fn update_offer_draft(&mut self, account: &String, id: &String) -> Result<()> {
    diesel::update(offers)
    .filter(offer_id.eq(id))
    .filter(account_id.eq(account))
    .set(draft.eq(false))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
}
