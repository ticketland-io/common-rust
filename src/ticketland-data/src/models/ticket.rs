use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::tickets;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = tickets)]
pub struct Ticket {
  pub ticket_nft: String,
  pub event_id: String,
  pub account_id: String,
  pub created_at: Option<NaiveDateTime>,
  pub ticket_type_index: i16,
  pub seat_name: String,
  pub seat_index: i32,
  pub attended: bool,
}
