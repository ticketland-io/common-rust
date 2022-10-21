use std::future::Future;
use eyre::Result;
use tokio::time::timeout as TokioTimeout;
use std::time::Duration;

pub async fn timeout<T, F>(millis: u64, future: F) -> Result<T>
where
    F: Future<Output=T>
{
  TokioTimeout(Duration::from_millis(millis), future)
  .await
  .map_err(Into::<_>::into)
}
