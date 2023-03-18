use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = accounts)]
pub struct Account {
  pub uid: String,
  pub created_at: Option<NaiveDateTime>,
  pub dapp_share: String,
  pub pubkey: String,
  pub name: Option<String>,
  pub email: Option<String>,
  pub photo_url: Option<String>,
}
