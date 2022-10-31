use rand::rngs::OsRng;
use eyre::Result;
use ed25519_dalek::{
  Keypair,
  PublicKey,
  Signature,
  Signer,
  Verifier,
};

pub fn create_keypair() -> Keypair {
  let mut csprng = OsRng{};
  Keypair::generate(&mut csprng)
}

pub fn sign(msg: &[u8], keypair: &[u8]) -> Result<Signature> {
  let keypair: Keypair = serde_json::from_slice(&keypair)?;
  Ok(keypair.sign(msg))
}

pub fn verify(msg: &[u8], pub_key: &[u8], signature: Signature) -> Result<()> {
  let public_key = PublicKey::from_bytes(pub_key)?;
  Ok(public_key.verify(msg, &signature)?)
}
