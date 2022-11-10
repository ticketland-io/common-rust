use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::canva_accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
// #[belongs_to(Account)]
#[diesel(table_name = canva_accounts)]
pub struct CanvaAccount {
  pub canva_uid: String,
  pub account_id: String,
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
}
