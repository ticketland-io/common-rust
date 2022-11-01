use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn read_api_client(client_id: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (ac:ApiClient {client_id: $client_id})
    RETURN ac{.*}
  "#;

  let params = create_params(vec![
    ("client_id", Value::String(client_id)),
  ]);

  (query, params)
}
