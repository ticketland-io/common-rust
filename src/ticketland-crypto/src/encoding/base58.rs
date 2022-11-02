use eyre::Result;

pub fn encode(data: &str) -> String {
  bs58::encode(data).into_string()
}
pub fn decode(data: &str) -> Result<Vec<u8>> {
  Ok(bs58::decode(data).into_vec()?)
}
