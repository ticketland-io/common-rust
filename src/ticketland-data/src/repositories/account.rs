use diesel::insert_into;
use eyre::Result;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use crate::{
  models::account::Account,
  schema::accounts::{self, name},
};

pub async fn upsert_account(conn: &mut AsyncPgConnection, account: Account) -> Result<()> {
  insert_into(accounts::table)
  .values(&account)
  .on_conflict(name)
  .do_update()
  .set(&account)
  .execute(conn)
  .await?;
  
  Ok(())
}
