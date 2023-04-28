use diesel::{prelude::*, upsert::excluded};
use diesel::result::Error;
use eyre::Result;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::{
    sale::{NewSale, Sale},
    seat_range::SeatRange,
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
  pub async fn upsert_sales(&mut self, event_id: String, sales_list: Vec<NewSale>, seat_ranges_list: Vec<SeatRange>) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      let existing_sales: Vec<Sale> = sales
      .filter(sales_dsl::event_id.eq(event_id))
      .select(sales::all_columns())
      .load::<Sale>(conn)
      .await?;

      // Sales in db that do not exist in `sales_list` should be deleted
      let sales_to_delete = existing_sales
      .iter()
      .filter_map(|existing_sale| {
        let should_remove = sales_list.iter().all(|sale| existing_sale.account != sale.account);
        if should_remove {
          Some(existing_sale.account.as_ref())
        } else {
          None
        }
      })
      .collect::<Vec<&str>>();
      diesel::delete(sales.filter(sales_dsl::account.eq_any(sales_to_delete)))
      .execute(conn)
      .await?;

      // find the existing sales and delete the corresponding seat_ranges
      let existing_sale_accounts = existing_sales.iter().map(|sale| sale.account.as_ref()).collect::<Vec<&str>>();
      diesel::delete(
        seat_ranges.filter(seat_ranges_dsl::sale_account.eq_any(existing_sale_accounts))
      )
      .execute(conn)
      .await?;

      // upsert the `sales_list` vector
      diesel::insert_into(sales)
      .values(&sales_list)
      .on_conflict(sales_dsl::account)
      .do_update()
      .set((
        sales_dsl::ticket_type_index.eq(excluded(sales_dsl::ticket_type_index)),
        sales_dsl::ticket_type_name.eq(excluded(sales_dsl::ticket_type_name)),
        sales_dsl::n_tickets.eq(excluded(sales_dsl::n_tickets)),
        sales_dsl::sale_start_ts.eq(excluded(sales_dsl::sale_start_ts)),
        sales_dsl::sale_end_ts.eq(excluded(sales_dsl::sale_end_ts)),
        sales_dsl::sale_type.eq(excluded(sales_dsl::sale_type)),
      ))
      .execute(conn)
      .await?;

      // upsert the `seat_ranges_list` vector
      diesel::insert_into(seat_ranges)
      .values(&seat_ranges_list)
      .on_conflict((seat_ranges_dsl::sale_account, seat_ranges_dsl::l,  seat_ranges_dsl::r))
      .do_nothing()
      .execute(conn)
      .await?;

      Ok(())
    }))
    .await?;

    Ok(())
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
