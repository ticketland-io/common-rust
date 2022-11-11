use eyre::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    seat_range::SeatRange,
  },
  schema::{
    seat_ranges::dsl::*,
  },
};

impl PostgresConnection {
  pub async fn read_event_seat_ranges(&mut self, s_account: String) -> Result<Vec<SeatRange>> {
    Ok(
      seat_ranges
      .filter(sale_account.eq(s_account))
      .load(self.borrow_mut())
      .await?
    )
  }
}
