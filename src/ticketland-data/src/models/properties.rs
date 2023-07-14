use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::properties;

#[derive(Queryable, QueryableByName, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = properties)]
pub struct Property {
  pub id: i32,
  pub nft_details_id: String,
  pub trait_type: String,
  pub value: String,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = properties)]
pub struct NewProperty {
  pub nft_details_id: String,
  pub trait_type: String,
  pub value: String,
}
