use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::canva_designs;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = canva_designs)]
pub struct CanvaDesign {
  pub design_id: String,
  pub canva_uid: String,
  pub created_at: Option<NaiveDateTime>,
  pub url: String,
  pub name: String,
  pub file_type: String,
}
