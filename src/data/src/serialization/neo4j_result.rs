use serde::ser::{Serialize, Serializer, SerializeSeq};
use crate::types::{Neo4jResult, WrappedValue};

impl Serialize for Neo4jResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

    for element in &self.0 {
      let wrapped = WrappedValue(element.clone());
      seq.serialize_element(&wrapped)?;
    }
  
    seq.end()
  }
}
