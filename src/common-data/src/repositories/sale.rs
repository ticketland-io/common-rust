use bolt_proto::value::{Value};
use bolt_client::{Params};
use ticketland_core::{
  actor::neo4j::{create_params}
};
use crate::models::sale::Sale;


pub fn upsert_event_sale(event_id: String, sales: Vec<Sale>) -> (&'static str, Option<Params>) {
  let query = r#"
    UNWIND $sales AS sale
    MATCH (evt:Event {event_id:$event_id})
    MERGE (evt)-[:HAS_SALE]->(s:Sale {
      account:sale.account,
      ticket_type_index:sale.ticket_type_index,
      ticket_type_name:sale.ticket_type_name,
      n_tickets:sale.n_tickets,
      sale_start_ts:sale.sale_start_ts,
      sale_end_ts:sale.sale_end_ts
    })
    WITH s, sale
    CALL apoc.do.when(
      EXISTS((:SeatRange)<-[:SEAT_RANGE]-(s)-[:HAS_TYPE]->(:SaleType)),
      '
      MATCH (sr:SeatRange)<-[:SEAT_RANGE]-(s)-[:HAS_TYPE]->(st:SaleType)
      SET st = $sale.sale_type
      SET sr = $sale.seat_range
      ',
      '
      CREATE (sr:SeatRange)<-[:SEAT_RANGE]-(s)-[:HAS_TYPE]->(st:SaleType)
      SET st = $sale.sale_type
      SET sr = $sale.seat_range
      ',
      {s:s, sale:sale}
    ) YIELD value
    RETURN 1
  "#;

  let sales: Vec<Value> = sales.iter().map(|s| Value::Map(s.to_neo4j_map())).collect();
  let params = create_params(vec![
    ("event_id", Value::String(event_id)),
    ("sales", Value::List(sales.into())),
  ]);

  (query, params)
}

pub fn read_event_sale(sale_account: String) -> (&'static str, Option<Params>) {
  let query = r#"
  MATCH (sr:SeatRange)<-[:SEAT_RANGE]-(s:Sale {account:$sale_account})-[:HAS_TYPE]->(st:SaleType)
  RETURN s {
    .*,
    seat_range: sr{.*},
    sale_type: st{.*}
  }
  "#;

  let params = create_params(vec![
    ("sale_account", Value::String(sale_account)),
  ]);

  (query, params)
}
