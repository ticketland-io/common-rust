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
  transaction::{Tag, Base64},
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
    tags: Option<Vec<(&str, &str)>>,
    reward_mult: f32,
    buffer: usize,
  ) -> CommandResult
  where
    IP: Iterator<Item = PathBuf> + Send + Sync
  {
    let tags = tags
      .map(|tags| {
        tags
        .into_iter()
        .map(|(name, value)| Tag {
          name: Base64(name.as_bytes().into()),
          value: Base64(value.as_bytes().into()),
        })
        .collect()
      });

    command_upload(
      &self.arweave,
      paths_iter,
      None,
      tags,
      reward_mult,
      &OutputFormat::Display,
      buffer,
    )
    .await?;

    Ok(())
  }
}
