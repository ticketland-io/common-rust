use sha3::{Sha3_256, Digest, digest::FixedOutput};

pub fn hash(msg: &[u8]) -> [u8; 32] {
  let mut hasher = Sha3_256::new();

  hasher.update(msg);

  <[u8; 32]>::from(hasher.finalize_fixed())
}
