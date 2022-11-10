use diesel::prelude::*;
use eyre::Result;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use crate::{
  models::account::Account,
  schema::accounts::dsl::*,
};

pub async fn upsert_account(conn: &mut AsyncPgConnection, account: Account) -> Result<()> {
  diesel::insert_into(accounts)
  .values(&account)
  .on_conflict(name)
  .do_update()
  .set(&account)
  .execute(conn)
  .await?;
  
  Ok(())
}

pub async fn read_account_by_id(conn: &mut AsyncPgConnection, user_id: String) -> Result<Account> {
  Ok(
    accounts
    .filter(uid.eq(user_id))
    .first(conn)
    .await?
  )
}
