use std::{
  str::FromStr,
  path::PathBuf,
};
use arloader::{
  upload_files_stream,
  Arweave,
  status::Status,
  error::Error,
  crypto::Provider,
  transaction::{Tag, Base64},
};
use ring::{
  rand,
  signature,
};
use futures::StreamExt;
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
    buffer: usize, // check here what this value means https://docs.rs/futures/0.2.1/futures/trait.StreamExt.html#method.buffer_unordered
  ) -> Result<Vec<Status>, Error>
  where
    IP: Iterator<Item = PathBuf> + Send + Sync,
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

      let price_terms = self.arweave.get_price_terms(reward_mult).await?;
      
      let mut stream = upload_files_stream(
        &self.arweave,
        paths_iter,
        tags,
        None,
        None,
        price_terms,
        buffer,
      );
      
    let mut statuses = vec![];

    while let Some(result) = stream.next().await {
      match result {
        Ok(status) => {
          statuses.push(status)
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    Ok(statuses)
  }
}
