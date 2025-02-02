use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_query};
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
    .on_conflict(uid)
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

  pub async fn update_delete_request_at(
    &mut self,
    user_id: String,
    delete_request_ts: Option<NaiveDateTime>,
  ) -> Result<()> {
    let query = sql_query(format!(
      "
      UPDATE accounts
      SET delete_request_at = {}
      WHERE uid = '{}';
      ",
      delete_request_ts.map_or("NULL".to_string(), |ts| format!("'{}'", ts.to_string())),
      user_id,
    ));

    query.execute(self.borrow_mut()).await?;

    Ok(())
  }

  pub async fn delete_account(
    &mut self,
    user_id: String,
  ) -> Result<()> {
    let query = sql_query(format!(
      "
      UPDATE accounts
      SET deleted_at = now()
      SET name = ''
      SET email = ''
      SET photo_url = ''
      WHERE uid = '{}';
      ",
      user_id,
    ));

    query.execute(self.borrow_mut()).await?;

    Ok(())
  }
}
