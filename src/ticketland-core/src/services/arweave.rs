use std::{
  str::FromStr,
  path::PathBuf,
};
use arloader::{
  Arweave,
  commands::*,
  error::Error,
  status::OutputFormat,
  crypto::Provider,
};
use ring::{
  rand,
  signature,
};
use jsonwebkey::JsonWebKey;
use url::Url;

pub struct Client {
  arweave: Arweave,
}

impl Client {
  pub async fn new(key_pair: String) -> Result<Self, Error> {
    let jwk_parsed: JsonWebKey = key_pair.parse().unwrap();
    let crypto = Provider {
      keypair: signature::RsaKeyPair::from_pkcs8(&jwk_parsed.key.as_ref().to_der())?,
      sr: rand::SystemRandom::new(),
    };

    let arweave = Arweave {
        base_url: Url::from_str("https://arweave.net").unwrap(),
        crypto,
        ..Default::default()
    };

    Ok(
      Self {
        arweave,
      }
    )
  }

  pub async fn upload<IP>(
    &self, 
    paths_iter: IP,
    reward_mult: f32,
    buffer: usize,
  ) -> CommandResult
  where
    IP: Iterator<Item = PathBuf> + Send + Sync
  {
    command_upload(
      &self.arweave,
      paths_iter,
      None,
      None,
      reward_mult,
      &OutputFormat::Display,
      buffer,
    )
    .await?;

    Ok(())
  }
}
