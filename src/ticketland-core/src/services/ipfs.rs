use std::io::Cursor;
use ipfs_api_backend_hyper::{
  IpfsClient,
  TryFromUri,
  IpfsApi,
  request::Add,
  response::AddResponse,
};
use crate::error::Error;

pub struct Ipfs {
  client: IpfsClient,
}

impl Ipfs {
  pub fn new(ipfs_server: String) -> Self {
    let client = IpfsClient::from_str(&ipfs_server).expect("cannot connect to ipfs server");
    
    Self {
      client,
    }
  }

  pub async fn dry_run(&self, data: Vec<u8>) -> Result<AddResponse, Error> {
    let mut options = Add::default();
    options.only_hash = Some(true);

    let data = Cursor::new(data);

    self.client.add_with_options(data, options)
    .await
    .map_err(Into::<Error>::into)
  }

  pub async fn upload(&self, data: Vec<u8>) -> Result<AddResponse, Error> {
    self.client.add(Cursor::new(data))
    .await
    .map_err(Into::<Error>::into)
  }
}
