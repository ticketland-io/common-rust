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

pub fn upsert_account(
  uid: String,
  email: String,
  name: String,
  photo_url: String,
  dapp_share: String,
  pubkey: String
) -> (&'static str, Option<Params>) {
  let query = r#"
    MERGE (acc:Account {uid: $uid})
    ON CREATE SET acc += {
      email:$email,
      name:$name,
      photo_url:$photo_url,
      dapp_share:$dapp_share,
      pubkey:$pubkey
    }
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("email", Value::String(email)),
    ("name", Value::String(name)),
    ("photo_url", Value::String(photo_url)),
    ("dapp_share", Value::String(dapp_share)),
    ("pubkey", Value::String(pubkey)),
  ]);

  (query, params)
}

pub fn create_canva_user(uid: String, canva_uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:IS_CANVA_USER]->(cu:CanvaUser {canva_uid: $canva_uid})
    RETURN cu{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("canva_uid", Value::String(canva_uid)),
  ]);

  (query, params)
}

pub fn read_account_by_canva_id(canva_uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account)-[:IS_CANVA_USER]->(cu:CanvaUser {canva_uid: $canva_uid})
    RETURN acc{.*}
  "#;

  let params = create_params(vec![
    ("canva_uid", Value::String(canva_uid)),
  ]);

  (query, params)
}
