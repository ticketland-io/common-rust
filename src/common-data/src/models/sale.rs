use std::{
  collections::HashMap,
  ops::Deref,
};
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
  pub fn to_map(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let name = match self {
      Self::Free => {
        map.insert("type".to_string(), "0".to_string());
      },
      Self::FixedPrice {price} => {
        map.insert("type".to_string(), "1".to_string());
        map.insert("price".to_string(), price.to_string());
      },
      Self::Refundable {price} => {
        map.insert("type".to_string(), "2".to_string());
        map.insert("price".to_string(), price.to_string());
      },
      Self::DutchAuction {start_price, end_price, curve_length, drop_interval} => {
        map.insert("type".to_string(), "2".to_string());
        map.insert("start_price".to_string(), start_price.to_string());
        map.insert("end_price".to_string(), end_price.to_string());
        map.insert("curve_length".to_string(), curve_length.to_string());
        map.insert("drop_interval".to_string(), drop_interval.to_string());
      },
    };
    
    map
  }
}

struct SaleTypeMap(HashMap<String, String>);

impl From<SaleType> for SaleTypeMap {
  fn from(sale_type: SaleType) -> Self {
    Self(sale_type.to_map())
  }
}

impl Deref for SaleTypeMap {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Serialize, Deserialize)]
pub struct SeatRange {
  pub l: u32,
  pub r: u32,  
}
