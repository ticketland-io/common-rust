use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

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

pub fn upsert_account_link(uid: String, stripe_uid: String, account_link: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:IS_STRIPE_USER]->(su:StripeUser {stripe_uid: $stripe_uid, status: 0})
    ON CREATE SET su.account_link = $account_link
    ON MATCH SET su.account_link = $account_link
    RETURN su{.*}
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("stripe_uid", Value::String(stripe_uid)),
    ("account_link", Value::String(account_link)),
  ]);

  (query, params)
}

pub fn update_stripe_account_status(stripe_uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (su:StripeUser {stripe_uid: $stripe_uid})
    SET su.status = 1
    REMOVE su.account_link
    RETURN 1
  "#;

  let params = create_params(vec![
    ("stripe_uid", Value::String(stripe_uid)),
  ]);

  (query, params)
}
