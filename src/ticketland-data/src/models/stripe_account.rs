use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::stripe_accounts;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = stripe_accounts)]
pub struct StripeAccount {
  pub stripe_uid: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub account_link: Option<String>,
  pub status: i16,
}
