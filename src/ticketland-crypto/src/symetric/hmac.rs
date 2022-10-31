use eyre::Result;
use ring::{
  digest,
  hmac,
  rand
};

pub fn sign_sha256(secret_key: &str, msg: &str) -> Result<String> {
  let key = base64::decode(secret_key)?;
  let s_key = hmac::Key::new(hmac::HMAC_SHA256, key.as_ref());
  let tag = hmac::sign(&s_key, msg.as_bytes());

  Ok(hex::encode(&tag.as_ref()))
}

pub fn create_key() -> Result<String>{
  let rng = rand::SystemRandom::new();
  let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();

  Ok(hex::encode(&key_value))
}
