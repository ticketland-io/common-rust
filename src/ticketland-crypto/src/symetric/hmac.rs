use eyre::Result;
use sha2::Sha256;
// use hmac::{Hmac, Mac};
use ring::{
  digest,
  hmac,
  rand
};

pub fn sign_sha256(secret_key: &str, msg: &str) -> Result<String> {
  let key = base64::decode(secret_key)?;
  let s_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key.as_ref());
  let tag = ring::hmac::sign(&s_key, msg.as_bytes());

  Ok(hex::encode(&tag.as_ref()))
}

// pub fn sign_sha256_2(secret_key: &str, message: &str) -> Result<String> {
//   let key = base64::decode(secret_key)?;
//   let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref()).unwrap();

//   mac.update(format!("{}", message).as_bytes());
//   Ok(hex::encode(&mac.finalize().into_bytes()[..]))
// }


pub fn create_key() -> Result<String>{
  let rng = ring::rand::SystemRandom::new();
  let key_value: [u8; ring::digest::SHA256_OUTPUT_LEN] = ring::rand::generate(&rng)?.expose();

  Ok(hex::encode(&key_value))
}
