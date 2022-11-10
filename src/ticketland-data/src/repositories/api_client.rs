use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    api_client::ApiClient,
  },
  schema::api_clients::dsl::*,
};

impl PostgresConnection {
  pub async fn read_api_client(&mut self, id: String) -> Result<ApiClient> {
    Ok(
      api_clients
      .filter(client_id.eq(id))
      .first(self.borrow_mut())
      .await?
    )
  }

}
