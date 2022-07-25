use s3::{
  bucket::Bucket,
  creds::Credentials,
  region::Region,
  error::S3Error,
};
use tokio::io::AsyncWrite;
pub struct Minio {
  bucket: Bucket,
}

impl Minio {
  pub async fn new(
    endpoint: &str,
    bucket_name: &str,
    access_key: &str,
    secret_key: &str,
  ) -> Self {
    let region = Region::Custom {
      region: "".into(),
      endpoint: endpoint.into(),
    };

    let credentials = Credentials::new(
      Some(&access_key),
      Some(&secret_key),
      None,
      None,
      None,
    ).expect("Wrong credentials provided");

    let bucket = Bucket::new(&bucket_name, region, credentials).expect("cannot init bucket");

    Self {
      bucket,
    }
  }

  pub async fn upload(&self, path: &str, content: &[u8]) -> Result<(), S3Error> {
    self.bucket.put_object(path, content).await?;
    
    Ok(())
  }

  pub async fn delete(&self, path: &str) -> Result<(), S3Error> {
    self.bucket.delete_object(path).await?;
    
    Ok(())
  }

  pub async fn get_object_stream<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
    &self,
    path: S,
    mut writer: &mut T
  ) -> Result<u16, S3Error> {
    Ok(
      self.bucket.get_object_stream(path, &mut writer).await?
    )
  }
}
