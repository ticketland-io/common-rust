use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    stripe_account::StripeAccount,
  },
  schema::{
    stripe_accounts::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn upsert_stripe_account(&mut self, account: StripeAccount) -> Result<()> {
    diesel::insert_into(stripe_accounts)
    .values(&account)
    .on_conflict(stripe_uid)
    .do_update()
    .set(&account)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
}
