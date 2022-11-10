use eyre::Result;
use diesel_async::RunQueryDsl;
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
}
