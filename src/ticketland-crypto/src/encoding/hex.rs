use eyre::Result;

pub fn encode(data: &[u8]) -> String {
  hex::encode(data)
}

pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
  Ok(hex::decode(data)?)
}
