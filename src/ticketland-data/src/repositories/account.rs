use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::account::Account,
  schema::accounts::dsl::*,
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
}
