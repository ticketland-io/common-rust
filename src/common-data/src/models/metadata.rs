use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
  trait_type: String,
  value: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Metadata {
  name: String,
  description: String,
  pub image: String,
  attributes: Vec<Attribute>,
}

impl Metadata {
  fn is_default(&self) -> bool {
    return self.name == ""
  