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

  fn create_tags(tags: Option<Vec<(&str, &str)>>) -> Option<Vec<Tag<Base64>>>{
    tags
    .map(|tags| {
      tags
      .into_iter()
      .map(|(name, value)| Tag {
        name: Base64(name.as_bytes().into()),
        value: Base64(value.as_bytes().into()),
      })
      .collect()
    })
  }

  /// Upload the given binary data to Arweave. This does not handle bundles and the max
  /// size should be MAX_TX_DATA 10mb.
  /// 
  /// # Arguments
  /// 
  /// * `data` - The binary data to upload
  /// * `other_tags` - optional additional tags to upload
  /// * `reward_mult` - used to calculate the price terms
  /// * `last_tx` - That last blockchain tx. If none it will fetch it from the blockchain
  /// * `auto_content_tag` - If true it will set the content-type tag which it will infer from the raw data (`infer::get(&data.0) {`)
  /// 
  /// * returns the transaction id as and the transaction reward
  pub async fn upload_data(
    &self,
    data: Vec<u8>,
    other_tags: Option<Vec<(&str, &str)>>,
    reward_mult: f32,
    last_tx: Option<Base64>,
    auto_content_tag: bool,
  ) -> Result<(Base64, u64), Error> {
    let price_terms = self.arweave.get_price_terms(reward_mult).await?;

    let tx = self.arweave.create_transaction(
      data,
      Self::create_tags(other_tags),
      last_tx,
      price_terms, 
      auto_content_tag
    ).await?;

    let signed_tx = self.arweave.sign_transaction(tx)?;
    self.arweave.post_transaction(&signed_tx).await
  }

  pub async fn upload_file<IP>(
    &self, 
    paths_iter: IP,
    tags: Option<Vec<(&str, &str)>>,
    reward_mult: f32,
    buffer: usize, // check here what this value means https://docs.rs/futures/0.2.1/futures/trait.StreamExt.html#method.buffer_unordered
  ) -> Result<Vec<Status>, Error>
  where
    IP: Iterator<Item = PathBuf> + Send + Sync,
  {
      let price_terms = self.arweave.get_price_terms(reward_mult).await?;
      
      let mut stream = upload_files_stream(
        &self.arweave,
        paths_iter,
        Self::create_tags(tags),
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
