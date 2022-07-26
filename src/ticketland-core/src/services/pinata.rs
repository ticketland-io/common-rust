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
  pinata_api_key: String,
  pinata_api_secret: String,
}

impl Pinata {
  pub fn new(
    pinata_api_url: String,
    pinata_api_key: String,
    pinata_api_secret: String,
  ) -> Self {
    let pinata_client = Client::new();
    
    Self {
      pinata_api_url,
      pinata_client,
      pinata_api_key,
      pinata_api_secret,
    }
  }

  pub async fn upload<R>(&self, data_stream: ReaderStream<R>) -> Result<(), Error> 
  where 
    R: AsyncRead + Send + Sync + 'static
  {
    let form = Form::new()
    .part("file", Part::stream(Body::wrap_stream(data_stream)))
    .text("pinataOptions", "{\"cidVersion\": 1}")
    .text("pinataMetadata", "{\"name\": \"MyFile\", \"keyvalues\": {\"company\": \"Pinata\"}}");

    self.pinata_client.post(format!("{}/pinning/pinFileToIPFS", self.pinata_api_url))
    .header("pinata_api_key", self.pinata_api_key.clone())
    .header("pinata_secret_api_key", self.pinata_api_secret.clone())
    .multipart(form)
    .send()
    .await
    .map_err(Into::<Error>::into)?;

    Ok(())
  }
}
