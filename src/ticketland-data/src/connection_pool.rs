use diesel_async::{
  AsyncPgConnection, pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};
use eyre::Result;
use super::connection::PostgresConnection;

pub struct ConnectionPool(Pool<AsyncPgConnection>);

impl ConnectionPool {
  pub async fn new(db_uri: &str) -> Self {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_uri);
    let pool = Pool::builder(config).build().unwrap();

    Self(pool)
  }

  pub async fn connection(&self) -> Result<PostgresConnection> {
    let conn = self.0.get().await?;
    Ok(PostgresConnection::new(conn))
  }
}


