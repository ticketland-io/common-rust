use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
  pub trait_type: String,
  pub value: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Metadata {
  pub name: String,
  pub description: String,
  pub image: String,
  pub attributes: Vec<Attribute>,
}

impl Metadata {
  pub fn is_default(&self) -> bool {
    return self.name == ""
  }
}
  