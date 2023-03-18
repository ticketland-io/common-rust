use eyre::{Result, ErrReport};
use aes_gcm_siv::{
  aead::{Aead, KeyInit},
  Aes256GcmSiv, Nonce,
};

/// Encrypts the given message
/// 
/// # Arguments
/// 
/// * `key`: the symetric key
/// * `nonce`: unique nonce per message
/// * `plaintext`: the text to encrypt
pub fn encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<String> {
  let cipher = Aes256GcmSiv::new(key.into());
  let nonce = Nonce::from_slice(nonce);
  let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|error| ErrReport::msg(error.to_string()))?;

  Ok(hex::encode(ciphertext))
}
