use fireauth::FireAuth;

pub struct Store {
  pub firebase_auth: FireAuth,
}

impl Store {
  pub fn new(firebase_auth_key: String) -> Self {
    let firebase_auth = fireauth::FireAuth::new(firebase_auth_key.clone());

    Self {
      firebase_auth,
    }
  }
}
