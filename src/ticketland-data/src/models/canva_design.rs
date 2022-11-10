use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::{
  NaiveDateTime,
  naive::serde::ts_milliseconds::serialize as to_milli_ts,
};
use crate::schema::canva_designs;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = canva_designs)]
pub struct CanvaDesign {
  pub design_id: String,
  pub canva_uid: String,
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
  pub url: String,
  pub name: String,
  pub file_type: String,
}
