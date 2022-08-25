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
    MERGE (acc)-[:HAS_SELL_LISTING]->(sl:SellListing {
      ask_price:$ask_price,
      created_at:$created_at
    })-[:FOR]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("ask_price", Value::Integer(ask_price.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}

pub fn create_buy_listing(
  uid: String,
  event_id: String,
  bid_price: u64,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:HAS_BUY_LISTING]->(bl:BuyListing {
      bid_price:$bid_price,
      created_at:$created_at
    })-[:FOR_TICKET_OF]->(evt)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("bid_price", Value::Integer(bid_price.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}
