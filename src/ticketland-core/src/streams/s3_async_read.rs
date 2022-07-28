// use std::{
//   sync::{Arc, RwLock},
//   future::Future,
//   pin::Pin,
// };
// use futures::{
//   io::{AsyncRead, Result},
//   task::{Poll, Context},
// };

// pub struct S3AsyncRead;

// impl AsyncRead for S3AsyncRead {
//   fn poll_read(
//     self: Pin<&mut Self>,
//     cx: &mut Context<'_>,
//     buf: &mut [u8],
//   ) -> Poll<Result<usize>> {
//     todo!()
//   }
// }
