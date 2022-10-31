pub fn encode() -> String {
  let pub_key = base64::encode(client_id)?;
}

pub fn decode() -> String {
  let pub_key = base64::decode(client_id)?;
}
