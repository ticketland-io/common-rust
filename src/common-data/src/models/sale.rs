use std::collections::HashMap;
use serde_aux::prelude::*;
use serde::{Deserialize, Serialize};
use bolt_proto::value::{Value};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
  pub ticket_type_index: u8,
  pub n_tickets: u32,
  pub sale_start_ts: u32,
  pub sale_end_ts: u32,
  pub sale_type: SaleType,
  pub seat_range: SeatRange,
}

impl Sale {
  pub fn to_neo4j_map(&self) -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("ticket_type_index".to_string(), Value::Integer(self.ticket_type_index as i64));
    map.insert("n_tickets".to_string(), Value::Integer(self.n_tickets as i64));
    map.insert("sale_start_ts".to_string(), Value::Integer(self.sale_start_ts as i64));
    map.insert("sale_end_ts".to_string(), Value::Integer(self.sale_end_ts as i64));
    map.insert("sale_type".to_string(), Value::Map(self.sale_type.to_neo4j_map()));
    map.insert("seat_range".to_string(), Value::Map(self.seat_range.to_neo4j_map()));

    map
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SaleType {
  Free {},
  FixedPrice {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    price: u64
  },
  Refundable {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    price: u64
  },
  DutchAuction {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    start_price: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}

impl SaleType {
  pub fn to_neo4j_map(&self) -> HashMap<String, Value> {
    let mut map = HashMap::new();

     match self {
      Self::Free {} => {
        map.insert("type".to_string(), Value::Integer(0));
      },
      Self::FixedPrice {price} => {
        map.insert("type".to_string(), Value::Integer(1));
        map.insert("price".to_string(), Value::Integer(*price as i64));
      },
      Self::Refundable {price} => {
        map.insert("type".to_string(), Value::Integer(2));
        map.insert("price".to_string(), Value::Integer(*price as i64));
      },
      Self::DutchAuction {start_price, end_price, curve_length, drop_interval} => {
        map.insert("type".to_string(), Value::Integer(3));
        map.insert("start_price".to_string(), Value::Integer(*start_price as i64));
        map.insert("end_price".to_string(), Value::Integer(*end_price as i64));
        map.insert("curve_length".to_string(), Value::Integer(*curve_length as i64));
        map.insert("drop_interval".to_string(), Value::Integer(*drop_interval as i64));
      },
    };
    
    map
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeatRange {
  pub l: u32,
  pub r: u32,  
}

impl SeatRange {
  pub fn to_neo4j_map(&self) -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("l".to_string(), Value::Integer(self.l as i64));
    map.insert("r".to_string(), Value::Integer(self.r as i64));

    map
  }
}
