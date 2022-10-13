use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};


pub fn create_stripe_user(uid: String, stripe_uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:IS_STRIPE_USER]->(su:StripeUser {stripe_uid: $stripe_uid})
    RETURN su{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("stripe_uid", Value::String(stripe_uid)),
  ]);

  (query, params)
}

pub fn read_stripe_user(uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})-[:IS_STRIPE_USER]->(su:StripeUser)
    RETURN su{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
  ]);

  (query, params)
}

pub fn upsert_account_link(uid: String, account_link: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:IS_STRIPE_USER]->(su:StripeUser)
    ON CREATE SET su.account_link = $account_link
    ON MATCH SET su.account_link = $account_link
    RETURN su{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("account_link", Value::String(account_link)),
  ]);

  (query, params)
}
