/// Note! this is not a Diesel model. We don't have such table at the moment in the db
use std::{
  collections::HashMap,
  ops::Deref,
};
use serde::{Serialize, Deserialize};
use serde_aux::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
  pub trait_type: String,
  #[serde(deserialize_with = "deserialize_string_from_number")]
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

  pub fn to_map(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("name".to_string(), self.name.clone());
    map.insert("description".to_string(), self.description.clone());
    map.insert("image".to_string(), self.image.clone());
    
    for attr in &self.attributes {
      map.insert(attr.trait_type.clone(), attr.value.clone());
    }

    map
  }
}

struct MetadataMap(HashMap<String, String>);

impl From<Metadata> for MetadataMap {
  fn from(metadata: Metadata) -> Self {
    Self(metadata.to_map())
  }
}

impl Deref for MetadataMap {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
