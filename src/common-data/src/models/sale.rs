use std::{
  collections::HashMap,
  ops::Deref,
};
use bolt_proto::value::{Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sale {
  pub ticket_type_index: u8,
  pub n_tickets: u32,
  pub sale_start_ts: u32,
  pub sale_end_ts: u32,
  pub sale_type: SaleType,
  pub seat_range: SeatRange,
}

#[derive(Serialize, Deserialize)]
pub enum SaleType {
  Free,
  FixedPrice {price: u64},
  Refundable {price: u64},
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}

impl SaleType {
  pub fn to_map(&self) -> HashMap<String, Value> {
    let mut map = HashMap::new();

     match self {
      Self::Free => {
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

struct SaleTypeMap(HashMap<String, Value>);

impl From<SaleType> for SaleTypeMap {
  fn from(sale_type: SaleType) -> Self {
    Self(sale_type.to_map())
  }
}

impl Deref for SaleTypeMap {
  type Target = HashMap<String, Value>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Serialize, Deserialize)]
pub struct SeatRange {
  pub l: u32,
  pub r: u32,  
}
