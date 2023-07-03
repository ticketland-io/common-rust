use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize};
use diesel::{prelude::*, sql_types};

#[derive(QueryableByName, Serialize)]
pub struct ClosedSalesData {
  #[diesel(sql_type = sql_types::BigInt)]
  count: i64,
  #[diesel(sql_type = sql_types::Timestamptz)]
  timestamp: NaiveDateTime,
}

#[derive(QueryableByName, Serialize)]
pub struct AverageSalesPrice {
  #[diesel(sql_type = sql_types::BigInt)]
  count: i64,
  #[diesel(sql_type = sql_types::Timestamptz)]
  timestamp: NaiveDateTime,
  #[diesel(sql_type = sql_types::Numeric)]
  avg: BigDecimal,
}
