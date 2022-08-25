use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn read_events(skip: u32, limit: u32) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event)
    RETURN evt{.*}
    ORDER BY evt.created_at DESC
    SKIP $skip
    LIMIT $limit
  "#;

  let skip = skip * limit;
  let params = create_params(vec![
    ("skip", Value::Integer((skip as i32).into())),
    ("limit", Value::Integer((limit as i32).into())),
  ]);

  (query, params)
}

pub fn read_event(event_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id: $event_id})
    RETURN evt{.*}
  "#;

  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}

pub fn read_account_events(uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})-[:ORGANIZER_OF]->(evt:Event)
    RETURN evt{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
  ]);

  (query, params)
}

pub fn upsert_event(
  event_id: String,
  event_organizer_uid: String,
  file_type: String,
  metadata_cid: String,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $event_organizer_uid})
    MERGE (acc)-[:ORGANIZER_OF]->(evt:Event {
      event_id:$event_id,
      file_type:$file_type,
      metadata_cid:$metadata_cid,
      metadata_uploaded: false,
      image_uploaded: false,
      created_at:$created_at
    })
    RETURN evt{.*}
  "#;

  let params = create_params(vec![
    ("event_organizer_uid", Value::String(event_organizer_uid)),
    ("event_id", Value::String(event_id)),
    ("file_type", Value::String(file_type)),
    ("metadata_cid", Value::String(metadata_cid)),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}

pub fn update_metadata_uploaded(event_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    SET evt.metadata_uploaded = true
    RETURN 1
  "#;

  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}

pub fn update_image_uploaded(event_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    SET evt.image_uploaded = true
    RETURN 1
  "#;

  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}

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
    MERGE (acc)-[:HAS_TICKET {owner: true}]->(t:Ticket {
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
