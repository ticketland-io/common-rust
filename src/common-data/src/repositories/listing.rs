use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn create_sell_listing(
  uid: String,
  event_id: String,
  ask_price: u64,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
  
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("ticket_nft", Value::String(ticket_nft)),
    ("ask_price", Value::Integer(seat_index.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}
