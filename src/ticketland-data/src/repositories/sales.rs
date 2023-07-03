use diesel::sql_query;
use eyre::Result;
use diesel_async::{RunQueryDsl};
use crate::models::sales::{ClosedSalesData, AverageSalesPrice};
use crate::{
  connection::PostgresConnection,
};

impl PostgresConnection {
  pub async fn read_closed_sales_count(
    &mut self,
    event_id: String,
    interval: u64,
    start_ts: u64
  ) -> Result<Vec<ClosedSalesData>> {
    let query = sql_query(format!(
      "
      SELECT date_bin(
        INTERVAL '{1} seconds',
        closed_at,
        TO_TIMESTAMP({2})::date
      ) as timestamp, COUNT(t)
      FROM
      (
        SELECT closed_at, is_open
        FROM offers
        WHERE event_id = '{0}' AND is_open = false AND EXTRACT(epoch from closed_at) > {2}
        UNION ALL
        SELECT closed_at, is_open
        FROM listings
        WHERE event_id = '{0}' AND is_open = false AND EXTRACT(epoch from closed_at) > {2}
      ) t
      GROUP BY 1
      ORDER BY timestamp desc;
      ", event_id, interval, start_ts
    ));

    Ok(query.load::<ClosedSalesData>(self.borrow_mut()).await?)
  }

  pub async fn read_average_sales_price(
    &mut self,
    event_id: String,
    interval: u64,
    start_ts: u64
  ) -> Result<Vec<AverageSalesPrice>> {
    let query = sql_query(format!(
      "
      SELECT date_bin(
        INTERVAL '{1} seconds',
        closed_at,
        TO_TIMESTAMP({2})::date
      ) as timestamp, AVG(price), COUNT(t)
      FROM
      (
        SELECT bid_price as price, closed_at
        FROM offers
        WHERE event_id = '{0}' AND is_open = false AND EXTRACT(epoch from closed_at) > {2}
        UNION ALL
        SELECT ask_price as price, closed_at
        FROM listings
        WHERE event_id = '{0}' AND is_open = false AND EXTRACT(epoch from closed_at) > {2}
      ) t
      GROUP BY 1
      ORDER BY timestamp desc;
      ", event_id, interval, start_ts
    ));

    Ok(query.load::<AverageSalesPrice>(self.borrow_mut()).await?)
  }
}
