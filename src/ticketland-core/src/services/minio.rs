use std::sync::{Arc};
use eyre::Result;
use s3::{
  bucket::Bucket,
  creds::Credentials,
  region::Region,
};
use tokio::{
  sync::RwLock,
  io::{AsyncRead, AsyncWrite},
};

pub struct Minio {
  bucket: Bucket,
}

impl Minio {
  pub async fn new(
    endpoint: Option<&str>,
    region: &str,
    bucket_name: &str,
    access_key: &str,
    secret_key: &str,
  ) -> Self {
    let credentials = Credentials::new(
      Some(&access_key),
      Some(&secret_key),
      None,
      None,
      None,
    ).expect("Wrong credentials provided");

    let bucket = if let Some(endpoint) = endpoint {
      // Use when target is custom Minio instance
      let region = Region::Custom {region: "".into(), endpoint: endpoint.into()};
      Bucket::new(&bucket_name, region, credentials).expect("cannot init bucket")
    } else {
      // Use when target is AWS
      let region = region.parse().unwrap();
      Bucket::new(&bucket_name, region, credentials).expect("cannot init bucket")
    };

    Self {
      bucket: bucket.with_path_style(),
    }
  }

  pub async fn upload(&self, path: &str, content: &[u8]) -> Result<()> {
    self.bucket.put_object(path, content).await?;
    
    Ok(())
  }

  pub async fn upload_with_content_type(&self, path: &str, content: &[u8], content_type: &str) -> Result<()> {
    self.bucket.put_object_with_content_type(path, content, content_type).await?;
    
    Ok(())
  }

  pub async fn delete(&self, path: &str) -> Result<()> {
    self.bucket.delete_object(path).await?;
    
    Ok(())
  }

  pub async fn get_object(&self, path: &str) -> Result<Vec<u8>> {
    Ok(self.bucket.get_object(path).await?.bytes().to_vec())
  }

  pub async fn get_object_stream<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
    self: Arc<Self>,
    path: S,
    writer: Arc<RwLock<T>>,
  ) -> Result<u16> {
    let mut writer = writer.write().await;

    Ok(
      self.bucket.get_object_stream(path, &mut *writer).await?
    )
  }

  pub async fn put_object_stream<T: AsyncRead + Send + Unpin, S: AsRef<str>>(
    self: Arc<Self>,
    path: S,
    reader: Arc<RwLock<T>>,
  ) -> Result<u16> {
    let mut reader = reader.write().await;

    Ok(
      self.bucket.put_object_stream(&mut *reader, path).await?
    )
  }
}
