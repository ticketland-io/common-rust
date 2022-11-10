use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::seat_ranges;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = seat_ranges)]
pub struct SeatRange {
  pub id: i32,
  pub sale_id: String,
  pub l: i32,
  pub r: i32,
}
