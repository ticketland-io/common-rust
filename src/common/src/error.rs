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
}

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
