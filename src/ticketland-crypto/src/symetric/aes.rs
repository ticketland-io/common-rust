use eyre::{Result, ErrReport};
use aes_gcm_siv::{
  aead::{Aead, KeyInit},
  Aes256GcmSiv, Nonce, Key
};

/// Encrypts the given message
/// 
/// # Arguments
/// 
/// * `key`: the symetric key
/// * `nonce`: unique nonce per message
/// * `plaintext`: the text to encrypt
pub fn encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<String> {
  let key = Key::<Aes256GcmSiv>::from_slice(key);
  let cipher = Aes256GcmSiv::new(key);
  let nonce = Nonce::from_slice(nonce);
  let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|error| ErrReport::msg(error.to_string()))?;

  Ok(hex::encode(ciphertext))
}

/// Decrypts the given ciphertext
/// 
/// # Arguments
/// 
/// * `key`: the symetric key
/// * `nonce`: unique nonce per message
/// * `ciphertext`: the ciphertext to decrypt
pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<String> {
  let key = Key::<Aes256GcmSiv>::from_slice(key);
  let cipher = Aes256GcmSiv::new(key);
  let nonce = Nonce::from_slice(nonce);
  let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|error| ErrReport::msg(error.to_string()))?;
  let plaintext = hex::decode(plaintext)?;

  Ok(String::from_utf8(plaintext)?)
}
