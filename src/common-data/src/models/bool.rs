use crate::types::Neo4jResult;
use std::convert::TryFrom;

impl TryFrom<Neo4jResult> for bool {
  type Error = ();

  fn try_from(v: Neo4jResult) -> Result<Self, Self::Error> {
    let value = v.0.get(0).unwrap().clone();
    let value = bool::try_from(value).expect("cannot convert value to boolean");

    Ok(value)
  }
}
