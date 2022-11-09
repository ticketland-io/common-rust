use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::accounts;

#[derive(Insertable, Queryable, Serialize, Deserialize, Default)]
#[diesel(table_name = accounts)]
pub struct Account {
  pub uid: String,
  pub mnemonic: String,
  pub pubkey: String,
  pub email: Option<String>,
  pub name: Option<String>,
  pub photo_url: Option<String>,
}
