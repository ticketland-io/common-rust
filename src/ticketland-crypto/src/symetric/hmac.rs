use eyre::Result;
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub fn sign_sha256(secret_key: &str, message: &str) -> Result<String> {
  let key = base64::decode(secret_key)?;
  let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref()).unwrap();

  mac.update(format!("{}", message).as_bytes());
  Ok(hex::encode(&mac.finalize().into_bytes()[..]))
}
