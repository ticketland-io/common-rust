/// Note! this is not a Diesel model. We don't have such table at the moment in the db
use std::{
  collections::HashMap,
  ops::Deref,
};
use serde::{Serialize, Deserialize};
use serde_aux::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TicketImage {
  pub ticket_image_type: i16,
  // pub ticket_type_index: i16,
  // pub ticket_nft_index: i16,
  // pub name: String,
  // pub description: String,
  pub path: String,
}

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
  pub images: Vec<TicketImage>,
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
    // map.insert("image".to_string(), self.image.clone());

    for image in &self.images {
      map.insert(image.ticket_image_type.to_string(), image.path.clone());
    }

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
