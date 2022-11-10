use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    canva_design::CanvaDesign,
  },
  schema::canva_designs::dsl::*,
};

impl PostgresConnection {
  pub async fn upsert_ticket_design(&mut self, design: CanvaDesign) -> Result<()> {
    diesel::insert_into(canva_designs)
    .values(&design)
    .on_conflict(design_id)
    .do_update()
    .set(&design)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
}
