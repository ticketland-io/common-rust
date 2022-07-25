use actix::prelude::*;
use std::sync::{Arc};
use std::iter::FromIterator;
use tokio::sync::Mutex;
use tokio::io::BufStream;
use tokio_util::compat::*;
use std::convert::TryFrom;
use bolt_client::{Client, Params, Metadata, Stream};
use bolt_proto::{message::*, version::*, Message, Value};
use crate::async_helpers::with_retry_panic;
use crate::error::Error;

#[derive(actix::Message, Debug)]
#[rtype(result = "Result<(Vec<Record>, Message), Error>")]
pub struct Read {
  pub query: String,
  pub params: Option<Params>,
  pub metadata: Option<Vec<(String, Value)>>
}

#[derive(actix::Message, Debug)]
#[rtype(result = "Result<(Vec<Record>, Message), Error>")]
pub struct Write {
  pub query: String,
  pub params: Option<Params>,
  pub metadata: Option<Vec<(String, Value)>>
}

type Neo4jClient = Client<Compat<BufStream<Stream>>>;

pub struct Neo4jActor {
  client: Arc<Mutex<Neo4jClient>>,
  database: Option<String>,
}

impl Neo4jActor {
  pub async fn new(
    neo4j_host: String,
    neo4j_domain: Option<String>,
    username: String,
    password: String,
    database: Option<String>,
  ) -> Self {
    let stream = Stream::connect(neo4j_host, neo4j_domain).await.expect("cannot connect to neo4j stream");
    let stream = BufStream::new(stream).compat();

    // Create a new connection to the server and perform a handshake to establish a
    // protocol version. This example demonstrates usage of the v4.2 or v4.3 protocol.
    let mut client = Client::new(stream, &[V4_3, V4_2, 0, 0]).await.expect("cannot create new bolt client");

    // Send a HELLO message with authorization details to the server to initialize
    // the session.
    let response = client.hello(
      Metadata::from_iter(vec![
        ("user_agent", "cross-pool-consumer/1.0"),
        ("scheme", "basic"),
        ("principal", &username),
        ("credentials", &password),
      ])).await.expect("cannot init bolt session");

    assert!(Success::try_from(response).is_ok());

    Neo4jActor {
      client: Arc::new(Mutex::new(client)),
      database,
    }
  }

  async fn run(
    client: Arc<Mutex<Neo4jClient>>,
    database: Option<String>,
    query: String,
    params: Option<Params>,
    metadata: Option<Vec<(String, Value)>>,
  ) -> Result<(Vec<Record>, Message), Error> {
    let client_cp = Arc::clone(&client);
    let database = ("db".to_owned(), Value::String(database.unwrap_or("neo4j".to_owned()).into()));
    let metadata = metadata
      .map(|mut metadata| {
        metadata.push(database.clone());
        Metadata::from_iter(metadata)
      })
      .unwrap_or(Metadata::from_iter(vec![database]));

    let action = || {
      async {
        let mut client_lock = client_cp.lock().await;
        let _ = client_lock.run(query.clone(), params.clone(), Some(metadata.clone()))
          .await
          .map_err(|error| Error::Neo4jError(format!("{:?}", error)));

        // Note: In version 4, note that we must pass metadata to PULL to indicate how many records we wish to consume
        // We use a big number by default
        let pull_meta = Metadata::from_iter(vec![("n", 1000)]);

        client_lock.pull(Some(pull_meta.clone()))
          .await
          .map_err(|error| Error::Neo4jError(format!("{:?}", error)))
      }
    };

    with_retry_panic(None, None, action).await
  }
}

impl Actor for Neo4jActor {
  type Context = Context<Self>;
}

impl Handler<Read> for Neo4jActor {
  type Result = ResponseFuture<Result<(Vec<Record>, Message), Error>>;

  fn handle(&mut self, msg: Read, _: &mut Self::Context) -> Self::Result {
    let client = Arc::clone(&self.client);
    let database = self.database.clone();

    let fut = async move {
      Self::run(
        client,
        database,
        msg.query,
        msg.params,
        msg.metadata
      ).await
    };

    Box::pin(fut)
  }
}

impl Handler<Write> for Neo4jActor {
  type Result = ResponseFuture<Result<(Vec<Record>, Message), Error>>;

  fn handle(&mut self, msg: Write, _: &mut Self::Context) -> Self::Result {
    let client = Arc::clone(&self.client);
    let database = self.database.clone();

    let fut = async move {
      Self::run(
        client,
        database,
        msg.query,
        msg.params,
        msg.metadata
      ).await
    };

    Box::pin(fut)
  }
}

pub fn create_params(params: Vec<(&str, Value)>) -> Option<Params> {
  Some(Params::from_iter(params))
}
