use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn read_ticket_designs(uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
  MATCH (acc:Account {uid: $uid})-[:DESIGNED]->(td:TicketDesign)
  RETURN td{.*}
"#;

let params = create_params(vec![
  ("uid", Value::String(uid)),
]);

(query, params)
}

pub fn upsert_ticket_design(
  canva_uid: String,
  design_id: String,
  url: String,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (cu:CanvaUser {canva_uid:$canva_uid})
    MERGE (td:TicketDesign {design_id: $design_id})
    MERGE (cu)-[:DESIGNED]->(td)
    ON CREATE SET td += {
      url:$url,
      created_at:$created_at
    }
    ON MATCH SET td += {
      url:$url
    }
    RETURN td{.*}
  "#;

  let params = create_params(vec![
    ("canva_uid", Value::String(canva_uid)),
    ("design_id", Value::String(design_id)),
    ("url", Value::String(url)),
    ("created_at", Value::Integer(created_at.into()))
  ]);

  (query, params)
}
