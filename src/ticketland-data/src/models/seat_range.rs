use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::seat_ranges;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = seat_ranges)]
pub struct SeatRange {
  pub sale_account: String,
  pub l: i32,
  pub r: i32,
}
