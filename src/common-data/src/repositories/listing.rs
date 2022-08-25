use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn create_sell_listing(
  uid: String,
  ticket_metadata: String,
  sell_listing_account: String,
  ask_price: u64,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MATCH (t:Ticket {ticket_metadata:$ticket_metadata})
    MERGE (acc)-[:HAS_SELL_LISTING {open: true}]->(sl:SellListing {
      sell_listing_account:sell_listing_account,
      ask_price:$ask_price,
      created_at:$created_at
    })-[:FOR]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("sell_listing_account", Value::String(sell_listing_account)),
    ("ask_price", Value::Integer(ask_price.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}

pub fn create_buy_listing(
  uid: String,
  event_id: String,
  buy_listing_account: String,
  bid_price: u64,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:HAS_BUY_LISTING]->(bl:BuyListing {
      buy_listing_account:$buy_listing_account,
      bid_price:$bid_price,
      created_at:$created_at
    })-[:FOR_TICKET_OF]->(evt)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("buy_listing_account", Value::String(buy_listing_account)),
    ("bid_price", Value::Integer(bid_price.into())),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}

// this will close the sell listing and will create a new rel that indicates the new owner of the ticket
// while invalidating the old owner. This way we can maintain the provenance of the ticket as well
pub fn fill_sell_listing(
  uid: String,
  sell_listing_account: String,
  ticket_metadata: String,
  created_at: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MATCH (acc)-[hsl:HAS_SELL_LISTING {open: true}]->(sl:SellListing {sell_listing_account:sell_listing_account})
    MATCH (:Account)-[ht:HAS_TICKET {owner: true}]->(t:Ticket {ticket_metadata:$ticket_metadata})
    SET hsl.open = false
    SET ht.owner = false
    CREATE (acc)-[ht:HAS_TICKET {owner: true, created_at:$created_at}]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("sell_listing_account", Value::String(sell_listing_account)),
    ("ticket_metadata", Value::String(ticket_metadata)),
    ("created_at", Value::Integer(created_at.into())),
  ]);

  (query, params)
}
