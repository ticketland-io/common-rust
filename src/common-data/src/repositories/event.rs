use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn read_event(event_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id: $event_id})
    RETURN acc{.*}
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
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $event_organizer_uid})
    MERGE (acc)-[:ORGANIZER_OF]->(evt:Event {event_id: $event_id})
    RETURN evt{.*}
  "#;

  let params = create_params(vec![
    ("event_organizer_uid", Value::String(event_organizer_uid)),
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}
