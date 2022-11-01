use std::ops::Deref;

use uuid::Uuid;
use crate::encoding::base58;

pub struct Id(String);

impl Id {
  pub fn new() -> Self {
    Self(base58::encode(&Uuid::new_v4().to_string()))
  }
}

impl Deref for Id {
  type Target = String;
  
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}
