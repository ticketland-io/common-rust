use fireauth::{
  FireAuth,
  api::User,
  Error,
};

pub struct FirebaseAuth {
  pub firebase_auth: FireAuth,
}

impl FirebaseAuth {
  pub fn new(api_key: String) -> Self {
    Self {
      firebase_auth: FireAuth::new(api_key)
    }
  }

  pub async fn get_user_info(&self, id_token: &str) -> Result<User, Error> {
    return self.firebase_auth.get_user_info(id_token).await
  }
}
