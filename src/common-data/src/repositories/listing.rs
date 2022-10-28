use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};

pub fn create_sell_listing(
  uid: String,
  ticket_nft: String,
  sell_listing_account: String,
  ask_price: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account {uid: $uid})
    MATCH (t:Ticket {ticket_nft:$ticket_nft})
    MERGE (acc)-[:HAS_SELL_LISTING {open: true}]->(sl:SellListing {
      account:$sell_listing_account,
      ask_price:$ask_price
    })-[:FOR]->(t)
    ON CREATE SET sl.created_at = timestamp()
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("ticket_metadata", Value::String(ticket_nft)),
    ("sell_listing_account", Value::String(sell_listing_account)),
    ("ask_price", Value::Integer(ask_price.into())),
  ]);

  (query, params)
}

pub fn read_sell_listing(sell_listing_account: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (sl:SellListing {account:$sell_listing_account})
    RETURN sl {.*}
  "#;

  let params = create_params(vec![
    ("sell_listing_account", Value::String(sell_listing_account)),
  ]);

  (query, params)
}

pub fn read_sell_listings_for_event(event_id: String, skip: u32, limit: u32) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (:Account)-[:HAS_SELL_LISTING {open: true}]->(sl:SellListing)-[:FOR]->(t:Ticket)-[:FROM]->(evt:Event {event_id:$event_id})
    RETURN sl{
      .*,
      metadata_cid: evt.metadata_cid,
      ticket_nft: t.ticket_nft,
      seat_index: t.seat_index,
      seat_name: t.seat_name
    } as result
    ORDER BY result.created_at DESC
    SKIP $skip
    LIMIT $limit
  "#;

  let skip = skip * limit;
  let params = create_params(vec![
    ("skip", Value::Integer((skip as i32).into())),
    ("limit", Value::Integer((limit as i32).into())),
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}

pub fn read_buy_listings_for_event(event_id: String, skip: u32, limit: u32) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (:Account)-[:HAS_BUY_LISTING {open: true}]->(bl:BuyListing)-[:FOR_TICKET_OF]->(evt:Event {event_id:$event_id})
    RETURN bl{
      .*,
      metadata_cid: evt.metadata_cid
    } as result
    ORDER BY result.created_at DESC
    SKIP $skip
    LIMIT $limit
  "#;

  let skip = skip * limit;
  let params = create_params(vec![
    ("skip", Value::Integer((skip as i32).into())),
    ("limit", Value::Integer((limit as i32).into())),
    ("event_id", Value::String(event_id)),
  ]);

  (query, params)
}

pub fn create_buy_listing(
  uid: String,
  event_id: String,
  buy_listing_account: String,
  bid_price: i64,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (evt:Event {event_id:$event_id})
    MATCH (acc:Account {uid: $uid})
    MERGE (acc)-[:HAS_BUY_LISTING {open: true}]->(bl:BuyListing {
      account:$buy_listing_account,
      bid_price:$bid_price,
    })-[:FOR_TICKET_OF]->(evt)
    ON CREATE SET bl.created_at = timestamp()
    RETURN 1
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
    ("event_id", Value::String(event_id)),
    ("buy_listing_account", Value::String(buy_listing_account)),
    ("bid_price", Value::Integer(bid_price.into())),
  ]);

  (query, params)
}

// this will close the sell listing and will create a new rel that indicates the new owner of the ticket
// while invalidating the old owner. This way we can maintain the provenance of the ticket as well
pub fn fill_sell_listing(
  ticket_buyer_uid: String,
  sell_listing_account: String,
  ticket_nft: String,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (ticket_buyer:Account {uid: $ticket_buyer_uid})
    MATCH (ticket_seller:Account)-[ht:HAS_TICKET {owner: true}]->(t:Ticket {ticket_nft:$ticket_nft})
    MATCH (ticket_seller)-[hsl:HAS_SELL_LISTING {open: true}]->(sl:SellListing {account:$sell_listing_account})
    SET hsl.open = false
    SET ht.owner = false
    CREATE (ticket_buyer)-[:HAS_TICKET {owner: true, created_at: timestamp()}]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("ticket_buyer_uid", Value::String(ticket_buyer_uid)),
    ("sell_listing_account", Value::String(sell_listing_account)),
    ("ticket_metadata", Value::String(ticket_nft)),
  ]);

  (query, params)
}

pub fn fill_buy_listing(
  ticket_seller_uid: String,
  buy_listing_account: String,
  ticket_nft: String,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (ticket_seller:Account {uid: $ticket_seller_uid})-[ht:HAS_TICKET {owner: true}]->(t:Ticket {ticket_nft:$ticket_nft})
    MATCH (ticket_buyer:Account)-[hbl:HAS_BUY_LISTING {open: true}]->(bl:BuyListing {account:$buy_listing_account})
    SET hbl.open = false
    SET ht.owner = false
    CREATE (ticket_buyer)-[ht:HAS_TICKET {owner: true, created_at: timestamp()}]->(t)
    RETURN 1
  "#;

  let params = create_params(vec![
    ("ticket_seller_uid", Value::String(ticket_seller_uid)),
    ("buy_listing_account", Value::String(buy_listing_account)),
    ("ticket_metadata", Value::String(ticket_nft)),
  ]);

  (query, params)
}

pub fn cancel_sell_listing(
  ticket_seller_uid: String,
  sell_listing_account: String,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (ticket_seller:Account {uid: $ticket_seller_uid})
    MATCH (ticket_seller)-[hsl:HAS_SELL_LISTING {open: true}]->(sl:SellListing {account:$sell_listing_account})
    SET hsl.open = false
    RETURN 1
  "#;

  let params = create_params(vec![
    ("ticket_seller_uid", Value::String(ticket_seller_uid)),
    ("sell_listing_account", Value::String(sell_listing_account)),
  ]);

  (query, params)
}

pub fn cancel_buy_listing(
  ticket_buyer_uid: String,
  buy_listing_account: String,
) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (ticket_buyer:Account {uid: $ticket_buyer_uid})
    MATCH (ticket_buyer)-[hbl:HAS_BUY_LISTING {open: true}]->(sl:BuyListing {account:$buy_listing_account})
    SET hbl.open = false
    RETURN 1
  "#;

  let params = create_params(vec![
    ("ticket_buyer_uid", Value::String(ticket_buyer_uid)),
    ("buy_listing_account", Value::String(buy_listing_account)),
  ]);

  (query, params)
}
