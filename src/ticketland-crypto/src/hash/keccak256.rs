use keccak_hash::keccak;

pub fn hash(msg: &[u8]) -> [u8; 32] {
  keccak(msg).0
}
