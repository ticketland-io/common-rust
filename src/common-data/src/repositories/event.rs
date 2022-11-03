use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn read_events_by_category(category: String, skip: u32, limit: u32) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {category: $category})
    WHERE evt.end_date > timestamp()
    MATCH (evt)-[:HAS_SALE]->(s)
    MATCH (s)-[:HAS_TYPE]->(st)
    WITH evt, COLLECT(s{.*, price: st.price}) as sales
    RETURN DISTINCT evt {
      .*,
      sales: sales
    }
    ORDER BY evt.start_date DESC
    SKIP $skip
    LIMIT $limit
  "#;

  let skip = skip * limit;
  let params = create_params(vec![
    ("skip", Value::Integer((skip as i32).into())),
    ("limit", Value::Integer((limit as i32).into())),
    ("category", Value::String(category))
  ]);

  (query, params)
}

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
    MATCH (acc:Account)-[:ORGANIZER_OF]->(evt:Event {event_id: $event_id})
    RETURN evt{
      .*,
      event_organizer: acc.pubkey
    }
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
  event_capacity: String,
  file_type: String,
  location: String,
  venue: String,
  event_type: String,
  start_date: String,
  end_date: String,
  category: String,
  name: String,
  description: String
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $event_organizer_uid})
    MERGE (acc)-[:ORGANIZER_OF]->(evt:Event {
      event_id:$event_id,
      event_capacity:$event_capacity,
      file_type:$file_type,
      metadata_uploaded: false,
      image_uploaded: false,
      location: $location,
      venue: $venue,
      event_type: $event_type,
      start_date: $start_date,
      end_date: $end_date,
      category: $category,
      name: $name,
      description: $description
    })
    ON CREATE SET evt.created_at = timestamp()
    RETURN evt{.*}
  "#;

  let params = create_params(vec![
    ("event_organizer_uid", Value::String(event_organizer_uid)),
    ("event_id", Value::String(event_id)),
    ("event_capacity", Value::String(event_capacity)),
    ("file_type", Value::String(file_type)),
    ("location", Value::String(location)),
    ("venue", Value::String(venue)),
    ("event_type", Value::String(event_type)),
    ("start_date", Value::Integer(start_date.parse::<i64>().unwrap())),
    ("end_date", Value::Integer(end_date.parse::<i64>().unwrap())),
    ("category", Value::String(category)),
    ("name", Value::String(name)),
    ("description", Value::String(description))
  ]);

  (query, params)
}

pub fn update_metadata_uploaded(event_id: String, arweave_tx_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    SET evt.metadata_uploaded = true
    SET evt.arweave_tx_id = $arweave_tx_id
    RETURN 1
  "#;

  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
    ("arweave_tx_id", Value::String(arweave_tx_id)),
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

pub fn read_event_organizer_account(event_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account)-[:ORGANIZER_OF]->(evt:Event {event_id:$event_id})
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}
