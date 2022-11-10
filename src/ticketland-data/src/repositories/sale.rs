use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    sale::Sale,
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
  },
};

impl PostgresConnection {
  pub async fn upsert_sales(&mut self, evt_id: String, sales_list: Vec<Sale>) -> Result<()> {
    Ok(())
  }
}
