use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    account::Account,
    canva_account::CanvaAccount,
  },
  schema::{
    accounts::dsl::*,
    canva_accounts::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn upsert_account(&mut self, account: Account) -> Result<()> {
    diesel::insert_into(accounts)
    .values(&account)
    .on_conflict(name)
    .do_update()
    .set(&account)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
  
  pub async fn read_account_by_id(&mut self, user_id: String) -> Result<Account> {
    Ok(
      accounts
      .filter(uid.eq(user_id))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_account_by_canva_id(&mut self, canva_id: String) -> Result<Account> {
    Ok(
      accounts
      .filter(canva_uid.eq(canva_id))
      .select(accounts::all_columns())
      .inner_join(canva_accounts.on(account_id.eq(uid)))
      .first::<Account>(self.borrow_mut())
      .await?
    )
  }

  pub async fn upsert_canva_account(&mut self, canva_account: CanvaAccount) -> Result<()> {
    diesel::insert_into(canva_accounts)
    .values(&canva_account)
    .on_conflict(canva_uid)
    .do_update()
    .set(&canva_account)
    .execute(self.borrow_mut())
    .await?;
    
    Ok(())
  }
}
