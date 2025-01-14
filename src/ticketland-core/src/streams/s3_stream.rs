use std::{
  sync::{Arc},
  future::Future,
  pin::Pin,
  io::{Error, ErrorKind},
  ops::DerefMut,
};
use eyre::Result;
use tokio::{
  sync::RwLock,
  io::{duplex, DuplexStream},
};
use tokio_util::io::ReaderStream;
use futures::{
  io::{AsyncRead},
  stream::Stream,
  task::{Poll, Context},
};
use bytes::Bytes;
use crate::{
  services::minio::Minio,
};

type ObjectStream = Pin<Box<dyn Future<Output = Result<u16>> + Send>>;

pub struct S3Stream {
  stream_reader: ReaderStream<DuplexStream>,
  get_object_stream: ObjectStream,
}

/// The `max_buf_size` argument is the maximum amount of bytes that can be
/// written to a side before the write returns `Poll::Pending`.
impl S3Stream {
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

impl Stream for S3Stream {
  type Item = Result<Bytes, Error>;

  fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let slf = DerefMut::deref_mut(&mut self);
    
    match Pin::new(&mut slf.stream_reader).poll_next(ctx) {
      Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error))),
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
          Err(error) => return Poll::Ready(Some(Err(Error::new(ErrorKind::Other, error.to_string())))),
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

impl AsyncRead for S3Stream {
  fn poll_read(
    mut self: Pin<&mut Self>,
    ctx: &mut Context<'_>,
    buf: &mut [u8],
  ) -> Poll<futures::io::Result<usize>> {
    let slf = DerefMut::deref_mut(&mut self);

    match Pin::new(&mut slf.stream_reader).poll_next(ctx) {
      Poll::Ready(Some(Err(error))) => return Poll::Ready(Err(error)),
      Poll::Ready(Some(Ok(data))) => {
        let len = data.len();
        data.into_iter().collect::<Vec<u8>>().append(&mut buf.to_vec());

        return Poll::Ready(Ok(len))
      },
      Poll::Ready(None) => return Poll::Ready(Ok(0)),
      // do nothing so we can move to the next part of the code
      Poll::Pending => {}, 
    };

    todo!()
  }
}
