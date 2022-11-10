use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    stripe_account::StripeAccount,
    account::Account,
  },
  schema::{
    stripe_accounts::dsl::*,
    accounts::dsl::{
      self as accounts_dsl,
      accounts,
    },
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

  pub async fn read_stripe_account(&mut self, user_id: String) -> Result<StripeAccount> {
    Ok(
      stripe_accounts
      .filter(account_id.eq(user_id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(account_id)))
      .first::<(StripeAccount, Account)>(self.borrow_mut())
      .await?
      .0
    )
  }
}
