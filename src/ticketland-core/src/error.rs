use actix_web::ResponseError;
use thiserror::Error;
use bolt_proto::Message;
use bolt_proto::message::{
  Failure,
  Record
};

#[derive(Error, Debug)]
pub enum Error {
  #[error("Neo4j error")]
  Neo4jError(String),
  #[error("Actor mailbox error")]
  MailboxError(String),
  #[error("No records found")]
  EmptyDbResult,
  #[error("S3 error")]
  S3Error(String),
  #[error("Generic error")]
  MultipartError(String),
}

impl ResponseError for Error {}

pub fn map_bolt_result_err(result: Result<(Vec<Record>, Message), Error>) -> Result<Vec<Record>, Error> {
  let (records, response) = result?;

  match response {
    bolt_proto::Message::Success(_) => Ok(records),
    bolt_proto::Message::Failure(error) => Err(Error::Neo4jError(format!("{:?}", error))),
    _ => Err(Error::Neo4jError(format!("Unknown error")))
  }
}

impl From<actix::MailboxError> for Error {
  fn from(error: actix::MailboxError) -> Self {
    Error::MailboxError(format!("{:?}", error))
  }
}

impl From<Failure> for Error {
  fn from(error: Failure) -> Self {
    Error::Neo4jError(format!("{:?}", error))
  }
}

impl From<s3::error::S3Error> for Error {
  fn from(error: s3::error::S3Error) -> Self {
    Error::S3Error(format!("{:?}", error))
  }
}

impl From<actix_multipart::MultipartError> for Error {
  fn from(error: actix_multipart::MultipartError) -> Self {
    Error::MultipartError(format!("{:?}", error))
  }
}
