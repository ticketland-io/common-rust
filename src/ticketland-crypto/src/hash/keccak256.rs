// TODO: Using solana sdk crate just for the keccak256 might be an overkill. Find an alternative keccak256 crate.
use solana_sdk::{
  keccak::hashv,
};

pub fn hash(msg: &[u8]) -> [u8; 32] {
  hashv(&[msg]).0
}
