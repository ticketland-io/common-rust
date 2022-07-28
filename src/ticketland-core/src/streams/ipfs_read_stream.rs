use std::{
  sync::{Arc, RwLock},
  future::Future,
  pin::Pin,
};
use tokio::io::{duplex, DuplexStream};
use tokio_util::io::ReaderStream;
use futures::{
  stream::Stream,
  task::{Poll, Context},
};
use bytes::Bytes;
use s3::error::S3Error;
use crate::{
  services::minio::Minio,
  error::Error,
};

type ObjectStream = Pin<Box<dyn Future<Output = Result<u16, S3Error>>>>;

pub struct IpfsReadStream {
  stream_reader: ReaderStream<DuplexStream>,
  get_object_stream: ObjectStream,
}

/// The `max_buf_size` argument is the maximum amount of bytes that can be
/// written to a side before the write returns `Poll::Pending`.
impl IpfsReadStream {
  pub fn new(
    file_path: String,
    max_buf_size: usize,
    minio: Arc<Minio>,
  ) -> Self {
    let (async_writer, async_reader) = duplex(max_buf_size);
    let async_writer = Arc::new(RwLock::new(async_writer));

    Self {
      stream_reader: ReaderStream::new(async_reader),
      get_object_stream: Box::pin(minio.get_object_stream(file_path.clone(), async_writer)),
    }
  }
}

impl Stream for IpfsReadStream {
  type Item = Result<Bytes, Error>;

  fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let slf = std::ops::DerefMut::deref_mut(&mut self);
    
    match Pin::new(&mut slf.stream_reader).poll_next(ctx) {
      // TODO: use a more specific error
      Poll::Ready(Some(Err(_))) => return Poll::Ready(Some(Err(Error::GenericError("error".to_owned())))),
      Poll::Ready(Some(Ok(data))) => return Poll::Ready(Some(Ok(data))),
      Poll::Ready(None) => return Poll::Ready(None),
      // do nothing so we can move to the next part of the code
      Poll::Pending => {}, 
    };

    let waker = ctx.waker().clone();

    match Future::poll(slf.get_object_stream.as_mut(), ctx) {
      Poll::Ready(status_code) => {
        match status_code {
          Ok(_) => return Poll::Ready(Some(Ok(Bytes::default()))),
          Err(error) => return Poll::Ready(Some(Err(error.into()))),
        }
        
      },
      Poll::Pending => {
        // This will happen max_buf_size if filled of the duplex async writer.
        // At this point call wake to move ons
        waker.wake();

        return Poll::Pending
      }
    }
  }
}