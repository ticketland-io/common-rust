use diesel::prelude::*;
use diesel::result::Error;
use eyre::Result;
use futures::FutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    sale::Sale,
    seat_range::SeatRange,
    event::Event,
  },
  schema::{
    sales::dsl::{
      self as sales_dsl,
      sales,
    },
    events::dsl::{
      self as events_dsl,
      events,
    },
    seat_ranges::dsl::{
      self as seat_ranges_dsl,
      seat_ranges,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_sales(&mut self, sales_list: Vec<Sale>, seat_ranges_list: Vec<SeatRange>) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| async move {
      diesel::insert_into(sales)
      .values(&sales_list)
      .on_conflict(sales_dsl::account)
      .do_nothing()
      .execute(conn)
      .await?;
      
      diesel::insert_into(seat_ranges)
      .values(&seat_ranges_list)
      .on_conflict((seat_ranges_dsl::sale_id, seat_ranges_dsl::l,  seat_ranges_dsl::r))
      .do_nothing()
      .execute(conn)
      .await?;

      Ok(())
    }.boxed())
    .await?;

    Ok(())
  }
}
