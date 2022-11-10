use diesel_async::{AsyncConnection, AsyncPgConnection};

pub struct PostgresConnection(AsyncPgConnection);

impl PostgresConnection {
  pub async fn new(db_uri: &str) -> Self {
    Self(AsyncPgConnection::establish(&db_uri).await.unwrap())
  }

  pub fn borrow(&mut self) -> &AsyncPgConnection {
    &self.0
  }

  pub fn borrow_mut(&mut self) -> &mut AsyncPgConnection {
    &mut self.0
  }
}
