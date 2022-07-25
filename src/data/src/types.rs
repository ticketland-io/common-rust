use std::convert::From;
use bolt_proto::{message::Record, value::Value};

pub struct WrappedValue(pub Value);

#[derive(Debug)]
pub struct Neo4jResult(pub Vec<Value>);

impl From<Vec<Record>> for Neo4jResult {
  fn from(records: Vec<Record>) -> Self {
    let mut vec = vec![];

    for record in records {
      let fields = record.fields()
        .iter()
        .fold(Vec::<Value>::new(), move |mut acc, field| {
          acc.push(field.clone().into());
          acc
        });

      vec.push(fields);
    }

    let result = vec
      .into_iter()
      .flatten()
      .collect();

    Neo4jResult(result)
  }
}
