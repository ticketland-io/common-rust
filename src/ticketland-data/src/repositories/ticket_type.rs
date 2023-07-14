use diesel::prelude::*;
use diesel::result::Error;
use eyre::Result;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    ticket_type::{NewTicketType, TicketType},
    seat_range::SeatRange,
  },
  schema::{
    ticket_types::dsl::{
      self as ticket_types_dsl,
      ticket_types,
    },
    seat_ranges::dsl::{
      self as seat_ranges_dsl,
      seat_ranges,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_ticket_types(&mut self, ticket_types_list: Vec<NewTicketType>, seat_ranges_list: Vec<SeatRange>) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      diesel::insert_into(ticket_types)
      .values(&ticket_types_list)
      .on_conflict((ticket_types_dsl::event_id, ticket_types_dsl::ticket_type_index))
      .do_nothing()
      .execute(conn)
      .await?;
      
      diesel::insert_into(seat_ranges)
      .values(&seat_ranges_list)
      .on_conflict((
        seat_ranges_dsl::event_id,
        seat_ranges_dsl::ticket_type_index,
        seat_ranges_dsl::l,
        seat_ranges_dsl::r
      ))
      .do_nothing()
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
  }
  
  pub async fn read_ticket_type(&mut self, event_id: String, ticket_type_index: i16) -> Result<TicketType> {
    Ok(
      ticket_types
      .filter(
        ticket_types_dsl::event_id.eq(event_id).and(
        ticket_types_dsl::ticket_type_index.eq(ticket_type_index))
      )
      .first(self.borrow_mut())
      .await?
    )
  }
  
  pub async fn read_ticket_type_by_sui_address(&mut self, ticket_type_sui_address: String) -> Result<TicketType> {
    Ok(
      ticket_types
      .filter(ticket_types_dsl::ticket_type_sui_address.eq(ticket_type_sui_address))
      .first(self.borrow_mut())
      .await?
    )
  }
}
