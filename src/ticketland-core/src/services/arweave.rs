use std::{
  str::FromStr,
  path::PathBuf,
};
use eyre::Result;
use arloader::{
  upload_files_stream,
  Arweave,
  bundle::DataItem,
  status::Status,
  crypto::Provider,
  transaction::{Tag, Base64, FromUtf8Strs},
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
  pub async fn new(key_pair: String) -> Result<Self> {
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
      .map(|(name, value)| Tag::<Base64>::from_utf8_strs(name, value).expect("valid tag"))
      .collect()
    })
  }

  fn create_tags_str(tags: Option<Vec<(&str, &str)>>) -> Option<Vec<Tag<String>>>{
    tags
    .map(|tags| {
      tags
      .into_iter()
      .map(|(name, value)| Tag::<String>::from_utf8_strs(name, value).expect("valid tag"))
      .collect()
    })
  }


  fn get_content_type(data: &Vec<u8>,) -> String {
    if let Some(kind) = infer::get(&data) {
      kind.mime_type().to_owned()
    } else {
        "application/octet-stream".to_owned()
    }
  }

  pub fn create_data_item(
    &self,
    data: Vec<u8>,
    other_tags: Option<Vec<(&str, &str)>>,
    auto_content_tag: bool,
  ) -> Result<(DataItem, Status)> {
    let content_type = Self::get_content_type(&data);
    let data_item = self.arweave.create_data_item(
      data,
      Self::create_tags_str(other_tags).unwrap_or(vec![]),
      auto_content_tag,
    )?;
    let data_item = self.arweave.sign_data_item(data_item)?;

    let status = Status {
      id: data_item.id.clone(),
      content_type,
      ..Status::default()
    };

    Ok((data_item, status))
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
  ) -> Result<(Base64, u64)> {
    let price_terms = self.arweave.get_price_terms(reward_mult).await?;

    let tx = self.arweave.create_transaction(
      data,
      Self::create_tags(other_tags),
      last_tx,
      price_terms, 
      auto_content_tag
    ).await?;

    let signed_tx = self.arweave.sign_transaction(tx)?;
    self.arweave.post_transaction(&signed_tx).await.map_err(Into::<_>::into)
  }

  /// Upload the given binary data to Arweave but unlike upload_data it will upload it as an Arweave Bundle.
  /// This does not handle bundles and the max
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
  pub async fn upload_file_as_bundle(
    &self,
    data: Vec<u8>,
    other_tags: Option<Vec<(&str, &str)>>,
    reward_mult: f32,
    auto_content_tag: bool,
  ) -> Result<(Base64, u64)> {
    let data_item = self.create_data_item(data, other_tags.clone(), auto_content_tag)?;

    let (bundle, _) = self.arweave.create_bundle_from_data_items(vec![data_item])?;
    let other_tags = other_tags.map(|mut tags| {
      tags.push(("Bundle-Format", "binary"));
      tags.push(("Bundle-Version", "2.0.0"));

      tags
    });
    
    let price_terms = self.arweave.get_price_terms(reward_mult).await?;
    let tx = self.arweave.create_transaction(
      bundle,
      Self::create_tags(other_tags),
      None,
      price_terms,
      true,
    )
    .await?;
    let signed_tx = self.arweave.sign_transaction(tx)?;

    self.arweave.post_transaction(&signed_tx).await.map_err(Into::<_>::into)
  }

  pub async fn upload_file<IP>(
    &self, 
    paths_iter: IP,
    tags: Option<Vec<(&str, &str)>>,
    reward_mult: f32,
    buffer: usize, // check here what this value means https://docs.rs/futures/0.2.1/futures/trait.StreamExt.html#method.buffer_unordered
  ) -> Result<Vec<Status>>
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
          return Err(error.into());
        },
      }
    }

    Ok(statuses)
  }
}
