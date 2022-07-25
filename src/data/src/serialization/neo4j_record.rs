use std::collections::HashMap;
use std::convert::TryFrom;
use serde::ser::{Serialize, Serializer, SerializeMap};
use bolt_proto::{value::Value};
use crate::types::{WrappedValue};

impl Serialize for WrappedValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    // TODO:: we use catchall for some of the value, but ultimately we should deal
    // with all enum values as documented here https://docs.rs/bolt-proto/0.10.0/bolt_proto/value/enum.Value.html
    match &self.0 {
      Value::Boolean(_) => serializer.serialize_bool(bool::try_from(self.0.clone()).unwrap()),
      Value::Integer(_) => serializer.serialize_i64(i64::try_from(self.0.clone()).unwrap()),
      Value::Float(_) => serializer.serialize_f64(f64::try_from(self.0.clone()).unwrap()),
      Value::String(_) => serializer.serialize_str(&String::try_from(self.0.clone()).unwrap()),
      Value::Map(_) => {
        let map = HashMap::<String, Value>::try_from(self.0.clone()).unwrap();
        let mut result = serializer.serialize_map(Some(map.len()))?;
        
        for (k, v) in map {
          result.serialize_entry(&k.to_string(), &WrappedValue(v))?;
        }
        
        result.end()
      },
      _ => serializer.serialize_str("")
    } 
  }
}
