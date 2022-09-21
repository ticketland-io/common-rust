use actix_web::ResponseError;
use thiserror::Error;
use ipfs_api_backend_hyper::Error as IpfsError;
use bolt_proto::Message;
use bolt_proto::message::{
  Failure,
  Record
};
use arloader::error::Error  as ArloaderError;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Generic Error")]
  GenericError(String),
  #[error("Neo4j error")]
  Neo4jError(String),
  #[error("Actor mailbox error")]
  MailboxError(String),
  #[error("No records found")]
  EmptyDbResult,
  #[error("S3 error")]
  S3Error(String),
  #[error("Multipart error")]
  MultipartError(String),
  #[error("Serde serialization or deserialization error")]
  SerdeJsonError(String),
  #[error("Ipfs error")]
  IpfsError(String),
  #[error("Reqwest Error")]
  ReqwestError(String),
  #[error("Stream Error")]
  StreamError(String),
  #[error("Arloder Error")]
  ArloaderError(String),
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

impl From<&str> for Error {
  fn from(error: &str) -> Self {
    Error::GenericError(format!("{:?}", error))
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
    println!("multipart error: {:?}", error);
    Error::MultipartError(format!("{:?}", error))
  }
}

impl From<serde_json::error::Error> for Error {
  fn from(error: serde_json::error::Error) -> Self {
    Error::SerdeJsonError(format!("{:?}", error))
  }
}

impl From<IpfsError> for Error {
  fn from(error: IpfsError) -> Self {
    Error::IpfsError(format!("{:?}", error))
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    Error::ReqwestError(format!("{:?}", error))
  }
}

impl From<ArloaderError> for Error {
  fn from(error: ArloaderError) -> Self {
    Error::ArloaderError(format!("{:?}", error))
  }
}
