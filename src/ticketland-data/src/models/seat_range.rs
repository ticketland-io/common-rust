use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::seat_ranges;

#[derive(Insertable, Queryable, AsChangeset, QueryableByName, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = seat_ranges)]
pub struct SeatRange {
  event_id: String,
  ticket_type_index: i16,
  pub l: i32,
  pub r: i32,
}
