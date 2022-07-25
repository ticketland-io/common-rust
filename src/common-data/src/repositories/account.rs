use bolt_proto::value::{Value};
use bolt_client::{Params};
use common::{
  actor::neo4j::{create_params}
};

pub fn read_account(uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
  ]);

  (query, params)
}

pub fn upsert_account(uid: String, mnemonic: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MERGE (acc:Account {uid: $uid})
    ON MATCH SET acc += {
      mnemonic:$mnemonic
    } 
    ON CREATE SET acc += {
      mnemonic:$mnemonic
    }
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("mnemonic", Value::String(mnemonic)),
  ]);

  (query, params)
}
