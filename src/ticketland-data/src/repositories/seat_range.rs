use eyre::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    seat_range::SeatRange,
  },
  schema::{
    seat_ranges::dsl::{
      self as seat_ranges_dsl,
      seat_ranges,
    }
  },
};

impl PostgresConnection {
  pub async fn read_ticket_type_seat_ranges(
    &mut self,
    event_id: String,
    ticket_type_index: i16,
  ) -> Result<Vec<SeatRange>> {
    Ok(
      seat_ranges
      .filter(seat_ranges_dsl::event_id.eq(event_id))
      .filter(seat_ranges_dsl::ticket_type_index.eq(ticket_type_index))
      .load(self.borrow_mut())
      .await?
    )
  }
}
