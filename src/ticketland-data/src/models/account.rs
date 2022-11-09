use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = accounts)]
pub struct Account {
  pub id: i32,
  pub uid: String,
  pub mnemonic: String,
  pub pubkey: String,
  pub name: Option<String>,
  pub email: Option<String>,
  pub photo_url: Option<String>,
}
