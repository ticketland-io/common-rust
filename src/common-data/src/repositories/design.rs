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
  uid: String,
  design_id: String,
  url: String,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MERGE (td:TicketDesign {design_id: $design_id})
    MERGE (acc)-[:DESIGNED]->(td)
    ON CREATE SET td += {
      url:$url
    }
    ON MATCH SET td += {
      url:$url
    }
    RETURN td{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("design_id", Value::String(design_id)),
    ("url", Value::String(url)),
  ]);

  (query, params)
}
