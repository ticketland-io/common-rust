use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn create_user_ticket(
  uid: String,
  event_id: String,
  ticket_nft: String,
  ticket_metadata: String,
  seat_index: u32,
  seat_name: String,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:HAS_TICKET {owner: true, created_at:$created_at}]->(t:Ticket {
      ticket_nft:$ticket_nft,
      ticket_metadata:$ticket_metadata,
      seat_index:$seat_index,
      seat_name:$seat_name,
      created_at:$created_at
    })-[:FROM]->(evt)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("ticket_nft", Value::String(ticket_nft)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("seat_index", Value::Integer(seat_index.into())),
    ("seat_name", Value::String(seat_name)),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}
