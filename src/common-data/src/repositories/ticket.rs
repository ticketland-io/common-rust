use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn upsert_user_ticket(
  uid: String,
  event_id: String,
  ticket_nft: String,
  ticket_metadata: String,
  seat_index: u32,
  seat_name: String,
  ticket_type_index: u8,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[ht:HAS_TICKET {owner: true}]->(t:Ticket {
      ticket_nft:$ticket_nft,
      ticket_metadata:$ticket_metadata,
      ticket_type_index:$ticket_type_index,
      seat_index:$seat_index,
      seat_name:$seat_name
    })-[:FROM]->(evt)
    ON CREATE SET
      ht.created_at = timestamp(),
      t.created_at = timestamp()
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("ticket_nft", Value::String(ticket_nft)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("seat_index", Value::Integer(seat_index.into())),
    ("seat_name", Value::String(seat_name)),
    ("ticket_type_index", Value::Integer(ticket_type_index.into())),

  ]);

  (query, params)
}


pub fn read_user_tickets_for_event(
  uid: String,
  event_id: String,
  skip: u32,
  limit: u32,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})-[:HAS_TICKET {owner: true}]->(t:Ticket)-[:FROM]->(evt:Event {event_id:$event_id})
    RETURN t{
      .*,
      arweave_tx_id: evt.arweave_tx_id
    }
  "#;

  let skip = skip * limit;
  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("skip", Value::Integer((skip as i32).into())),
    ("limit", Value::Integer((limit as i32).into())),
  ]);

  (query, params)
}

pub fn read_ticket_by_ticket_metadata(ticket_metadata: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (t:Ticket {ticket_metadata:$ticket_metadata})
    RETURN t{.*}
  "#;
  
  let params = create_params(vec![
    ("ticket_metadata", Value::String(ticket_metadata)),
  ]);

  (query, params)
}
