use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
  pub trait_type: String,
  pub value: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
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
  