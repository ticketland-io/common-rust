use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = accounts)]
pub struct Account {
  pub uid: String,
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
  pub mnemonic: String,
  pub pubkey: String,
  pub name: Option<String>,
  pub email: Option<String>,
  pub photo_url: Option<String>,
}
