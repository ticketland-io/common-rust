use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::api_clients;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = api_clients)]
pub struct ApiClient {
  pub client_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub client_secret: String,
}
