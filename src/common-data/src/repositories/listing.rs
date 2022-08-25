use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn create_sell_listing(
  uid: String,
  ticket_metadata: String,
  ask_price: u64,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MATCH (t:Ticket {ticket_metadata:$ticket_metadata})
    MERGE (acc)-[:HAS_SELL_LISTING]->(t:SellListing {
      ask_price:$ask_price,
      created_at:$created_at
    })-[:FOR]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("ask_price", Value::Integer(seat_index.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}
