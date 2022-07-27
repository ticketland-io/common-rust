use reqwest::{
  Client,
  Body,
  multipart::{Form, Part},
};
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;
use crate::error::Error;

pub struct Pinata {
  pinata_client: Client,
  pinata_api_url: String,
  pinata_api_token: String,
}

impl Pinata {
  pub fn new(
    pinata_api_url: String,
    pinata_api_token: String,
  ) -> Self {
    let pinata_client = Client::new();
    
    Self {
      pinata_api_url,
      pinata_client,
      pinata_api_token,
    }
  }

  pub async fn upload<R>(
    &self,
    file_name: &str,
    data_stream: ReaderStream<R>
  ) -> Result<(), Error> 
  where 
    R: 'static + AsyncRead + Send + Sync
  {
    let form = Form::new()
    .part("file", Part::stream(Body::wrap_stream(data_stream)))
    .text("pinataOptions", "{\"cidVersion\": 1}")
    .text("pinataMetadata", format!("{{\"name\": \"{}\"}}", file_name));

    self.pinata_client.post(format!("{}/pinning/pinFileToIPFS", self.pinata_api_url))
    .header("Authorization", format!("Bearer {}", self.pinata_api_token.clone()))
    .multipart(form)
    .send()
    .await
    .map_err(Into::<Error>::into)?;

    Ok(())
  }
}
