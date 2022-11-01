use eyre::Result;

pub fn encode(data: &str) -> String {
  bs58::encode(data)
}
pub fn decode(data: &str) -> Result<Vec<u8>> {
  Ok(bs58::decode(data)?)
}
