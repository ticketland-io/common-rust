use ipfs_api_backend_actix::{
  IpfsClient,
  TryFromUri, IpfsApi,
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
      client
    }
  }

  pub async fn calc_cid(&self, data: &'static [u8]) -> Result<AddResponse, Error> {
    let mut options = Add::default();
    options.only_hash = Some(true);

    self.client.add_with_options(data,options)
    .await
    .map_err(Into::<Error>::into)
  }
}
