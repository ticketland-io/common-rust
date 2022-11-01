use eyre::Result;

pub fn encode(data: &str) -> String {
  base64::encode(data)
}
pub fn decode(data: &str) -> Result<Vec<u8>> {
  Ok(base64::decode(data)?)
}
