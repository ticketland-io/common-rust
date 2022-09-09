use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
  trait_type: String,
  value: String,
}

#[derive(Default, Debug, Serialize)]
pub struct Metadata {
  name: String,
  description: String,
  image: String,
  attributes: Vec<Attribute>,
}
