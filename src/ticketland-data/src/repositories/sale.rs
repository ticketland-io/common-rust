use diesel::prelude::*;
use diesel::result::Error;
use eyre::Result;
use futures::FutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    sale::{NewSale, Sale},
    seat_range::NewSeatRange,
  },
  schema::{
    sales::dsl::{
      self as sales_dsl,
      sales,
    },
    seat_ranges::dsl::{
      self as seat_ranges_dsl,
      seat_ranges,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_sales(&mut self, sales_list: Vec<NewSale>, seat_ranges_list: Vec<NewSeatRange>) -> Result<()> {
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

  pub async fn read_sale_by_event(&mut self, evt_id: String) -> Result<Sale> {
    Ok(
      sales
      .filter(sales_dsl::event_id.eq(evt_id))
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_sale_by_account(&mut self, sale_account: String) -> Result<Sale> {
    Ok(
      sales
      .filter(sales_dsl::account.eq(sale_account))
      .first(self.borrow_mut())
      .await?
    )
  }
}
