use std::ops::{Deref, DerefMut};
use diesel_async::{AsyncConnection, AsyncPgConnection};

pub struct PostgresConnection(AsyncPgConnection);

impl Deref for PostgresConnection {
  type Target = AsyncPgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for PostgresConnection {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PostgresConnection {
  pub async fn new(db_uri: &str) -> Self {
    Self(AsyncPgConnection::establish(&db_uri).await.unwrap())
  }
}
