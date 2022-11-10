use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    canva_design::CanvaDesign,
  },
  schema::{
    canva_designs::dsl::{
      self as canva_designs_dsl,
      canva_designs,
    },
    canva_accounts::dsl::{
      self as canva_accounts_dsl,
      canva_accounts,
    },
  },
};

impl PostgresConnection {
  pub async fn upsert_ticket_design(&mut self, design: CanvaDesign) -> Result<()> {
    diesel::insert_into(canva_designs)
    .values(&design)
    .on_conflict(canva_designs_dsl::design_id)
    .do_update()
    .set(&design)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }

  pub async fn read_canva_designs(&mut self, account_id: String) -> Result<Vec<CanvaDesign>> {
    Ok(
      canva_accounts
      .filter(canva_accounts_dsl::account_id.eq(account_id))
      .inner_join(canva_designs.on(canva_designs_dsl::canva_uid.eq(canva_accounts_dsl::canva_uid)))
      .select(canva_designs::all_columns())
      .load::<CanvaDesign>(self.borrow_mut())
      .await?
    )
  }
}
