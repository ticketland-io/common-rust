///Example of usage
/// 
/// ```
/// let keypair = ed25519::create_keypair();
/// // We can store these values in plain text
/// let keypair = base64::encode(&serde_json::to_string(&keypair)?);
/// let pubkey = base64::encode(&serde_json::to_string(&keypair.public.as_ref())?);
/// 
/// // One service signs 
/// let msg = "some message";
/// let sig = ed25519::sign(msg.as_bytes(), keypair.as_bytes())?;
/// 
/// // Another service can verify the signature.
/// ed25519::verify(msg.as_bytes(), pubkey.as_bytes(), &sig.to_string())?;
use std::str::FromStr;
use rand::rngs::OsRng;
use eyre::Result;
use ed25519_dalek::{
  Keypair,
  PublicKey,
  Signature,
  Signer,
  Verifier,
};
use crate::hash::keccak256;

pub fn create_keypair() -> Keypair {
  let mut csprng = OsRng{};
  Keypair::generate(&mut csprng)
}

pub fn sign(msg: &[u8], keypair: &[u8]) -> Result<Signature> {
  let keypair = base64::decode(keypair)?;
  let keypair: Keypair = serde_json::from_slice(&keypair)?;
  let msg = keccak256::hash(msg);

  Ok(keypair.sign(msg.as_ref()))
}

pub fn verify(msg: &[u8], pub_key: &[u8], sig: &str) -> Result<()> {
  let pub_key = base64::decode(pub_key)?;
  let public_key: PublicKey = PublicKey::from_bytes(&pub_key)?;
  let msg = keccak256::hash(msg);

  Ok(public_key.verify(msg.as_ref(), &Signature::from_str(sig)?)?)
}
