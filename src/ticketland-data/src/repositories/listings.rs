use diesel::sql_query;
use eyre::Result;
use diesel_async::{RunQueryDsl};
use crate::models::listings::{ClosedListingsData, AverageListingsPrice};
use crate::{
  connection::PostgresConnection,
};

impl PostgresConnection {
  pub async fn read_closed_listings_count(&mut self, interval: u64, start_ts: u64) -> Result<Vec<ClosedListingsData>> {
    let query = sql_query(format!(
      "
      SELECT date_bin(
        INTERVAL '{0} seconds',
        closed_at,
        TO_TIMESTAMP({1})::date
      ) as timestamp, COUNT(t)
      FROM
      (
        SELECT closed_at, is_open
        FROM buy_listings
        WHERE is_open = false AND EXTRACT(epoch from closed_at) > {1}
        UNION ALL
        SELECT closed_at, is_open
        FROM sell_listings
        WHERE is_open = false AND EXTRACT(epoch from closed_at) > {1}
      ) t
      GROUP BY 1
      ORDER BY timestamp desc;
      ", interval, start_ts
    ));

    Ok(query.load::<ClosedListingsData>(self.borrow_mut()).await?)
  }

  pub async fn read_average_listings_price(&mut self, interval: u64, start_ts: u64) -> Result<Vec<AverageListingsPrice>> {
    let query = sql_query(format!(
      "
      SELECT date_bin(
        INTERVAL '{0} seconds',
        closed_at,
        TO_TIMESTAMP({1})::date
      ) as timestamp, AVG(price), COUNT(t)
      FROM
      (
        SELECT bid_price as price, closed_at
        FROM buy_listings
        WHERE is_open = false AND EXTRACT(epoch from closed_at) > {1}
        UNION ALL
        SELECT ask_price as price, closed_at
        FROM sell_listings
        WHERE is_open = false AND EXTRACT(epoch from closed_at) > {1}
      ) t
      GROUP BY 1
      ORDER BY timestamp desc;
      ", interval, start_ts
    ));

    Ok(query.load::<AverageListingsPrice>(self.borrow_mut()).await?)
  }
}
