use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct PostgresConnection(PgConnection);

impl PostgresConnection {
  pub fn new(db_uri: &str) -> Self {
    Self(PgConnection::establish(&db_uri).unwrap())
  }
}
