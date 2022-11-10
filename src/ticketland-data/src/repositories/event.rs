use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use eyre::Result;
use crate::{
  connection::PostgresConnection,
  models::{
    event::Event,
  },
  schema::{
    events::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn upsert_event(&mut self, event: Event) -> Result<()> {
    diesel::insert_into(events)
    .values(&event)
    .on_conflict(event_id)
    .do_update()
    .set(&event)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }

  pub async fn update_metadata_uploaded(&mut self, id: String, arweave_tx: String) -> Result<()> {
    diesel::update(events)
    .filter(event_id.eq(id))
    .set(arweave_tx_id.eq(arweave_tx))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn update_image_uploaded(&mut self, id: String) -> Result<()> {
    diesel::update(events)
    .filter(event_id.eq(id))
    .set(image_uploaded.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
}
