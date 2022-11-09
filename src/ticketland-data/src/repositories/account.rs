use diesel::insert_into;
use crate::{
  models::account::Account,
  schema::accounts,
};

pub fn upsert_account(account: Account) {
  insert_into(accounts)
  .values(&account)
}
