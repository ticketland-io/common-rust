use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::stripe_customers;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = stripe_customers)]
pub struct StripeCustomer {
  pub customer_uid: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
}
