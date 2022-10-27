use std::collections::HashMap;
use serde_aux::prelude::*;
use serde::{Deserialize, Serialize};
use bolt_proto::value::{Value};
use ticketland_core::error::Error;
use crate::types::Neo4jResult;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
  pub account: String,
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
    
    map.insert("account".to_string(), Value::String(self.account.clone()));
    map.insert("ticket_type_index".to_string(), Value::Integer(self.ticket_type_index as i64));
    map.insert("n_tickets".to_string(), Value::Integer(self.n_tickets as i64));
    map.insert("sale_start_ts".to_string(), Value::Integer(self.sale_start_ts as i64));
    map.insert("sale_end_ts".to_string(), Value::Integer(self.sale_end_ts as i64));
    map.insert("sale_type".to_string(), Value::Map(self.sale_type.to_neo4j_map()));
    map.insert("seat_range".to_string(), Value::Map(self.seat_range.to_neo4j_map()));

    map
  }
}

impl TryFrom<Neo4jResult> for Sale {
  type Error = Error;

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    if v.0.len() == 0 {
      return Err(Error::EmptyDbResult)
    }

    let value = v.0.get(0).unwrap().clone();

    let sale = match value {
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(value).expect("cannot convert value to map");
        let mut sale = Sale {
          ..Default::default()
        };

        for (k, v) in map {
          match k.as_str() {
            "account" => {
              sale.account = String::try_from(v).expect("cannot convert account");
            },
            "ticket_type_index" => {
              sale.ticket_type_index = i8::try_from(v).expect("cannot convert ticket_type_index") as u8;
            },
            "n_tickets" => {
              sale.n_tickets = i32::try_from(v).expect("cannot convert n_tickets") as u32;
            },
            "sale_start_ts" => {
              sale.sale_start_ts = i32::try_from(v).expect("cannot convert sale_start_ts") as u32;
            },
            "sale_end_ts" => {
              sale.sale_end_ts = i32::try_from(v).expect("cannot convert sale_end_ts") as u32;
            },
            "sale_type" => {
              let mut map = HashMap::<String, Value>::try_from(v).expect("cannot convert value to map");
              let s_type = i8::try_from(map.remove("type").unwrap()).expect("cannot convert sale_type") as u8;
              
              let sale_type = match s_type {
                0 => SaleType::Free {},
                1 => SaleType::FixedPrice { price: i64::try_from(map.remove("price").unwrap()).expect("cannot convert price") as u64 },
                2 => SaleType::Refundable { price: i64::try_from(map.remove("price").unwrap()).expect("cannot convert price") as u64 },
                3 => SaleType::DutchAuction {
                  start_price: i64::try_from(map.remove("start_price").unwrap()).expect("cannot convert start_price") as u64,
                  end_price: i64::try_from(map.remove("end_price").unwrap()).expect("cannot convert end_price") as u64,
                  curve_length: i16::try_from(map.remove("curve_length").unwrap()).expect("cannot convert curve_length") as u16,
                  drop_interval: i16::try_from(map.remove("drop_interval").unwrap()).expect("cannot convert drop_interval") as u16,
                },
                _ => panic!("Sale type not found"),
              };

              sale.sale_type = sale_type;
            },
            "seat_range" => {
              let mut map = HashMap::<String, Value>::try_from(v).expect("cannot convert value to map");
              sale.seat_range = SeatRange {
                l: i32::try_from(map.remove("l").unwrap()).expect("cannot convert l") as u32,
                r: i32::try_from(map.remove("r").unwrap()).expect("cannot convert r") as u32
              }
            },
            _ => panic!("unknown field"),
          }
        }

        sale
      }
      _ => panic!("neo4j result should be a Value::Map"),
    };

    Ok(sale)
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

impl Default for SaleType {
  fn default() -> Self {
    Self::Free {}
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

#[derive(Serialize, Deserialize, Default)]
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
