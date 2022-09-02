use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
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

pub fn upsert_account(uid: String, mnemonic: String, pubkey: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MERGE (acc:Account {uid: $uid})
    ON CREATE SET acc += {
      mnemonic:$mnemonic,
      pubkey:$pubkey
    }
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("mnemonic", Value::String(mnemonic)),
    ("pubkey", Value::String(pubkey)),
  ]);

  (query, params)
}
