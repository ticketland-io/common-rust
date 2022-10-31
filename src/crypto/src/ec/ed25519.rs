use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

pub fn create_keypair() -> Keypair {
  let mut csprng = OsRng{};
  Keypair::generate(&mut csprng)
}
