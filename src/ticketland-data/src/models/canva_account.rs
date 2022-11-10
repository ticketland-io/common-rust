use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::canva_accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = canva_accounts)]
pub struct CanvaAccount {
  pub canva_uid: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
}
