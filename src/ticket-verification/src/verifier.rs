use std::{sync::Arc, str::FromStr};
use chrono::Duration;
use eyre::{eyre, Result, ContextCompat};
use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize};
use sui_sdk::{SuiClient, rpc_types::{SuiObjectDataOptions, SuiData}};
use sui_types::base_types::ObjectID;
use ticketland_crypto::asymetric::ed25519;
use ticketland_core::services::{redis, redlock::RedLock};
use ticketland_data::{connection_pool::ConnectionPool, models::cnt::CNT};
use crate::error::Error;

// Executed the provided function and converts the Result into eyre::Result
#[macro_export]
macro_rules! map_err {
  ($fun:expr) => {
    $fun.map_err(|e| eyre!(Box::new(e)))
  }
}

// pub async fn get_cnt_object(
//   api: Arc<SuiClient>,
//   cnt_sui_address: &str,
// ) -> Result<CNT> {
//   let response = api
//   .read_api()
//   .get_object_with_options(
//     ObjectID::from_str(cnt_sui_address)?,
//     SuiObjectDataOptions::new().with_bcs(),
//   )
//   .await?;

//   let cnt: CNT = map_err!(response
//     .object()?
//     .bcs
//     .as_ref()
//     .context("Could not get ref")?
//     .try_as_move()
//     .context("Could not convert to MoveObject")?
//     .deserialize()
//   )?;

//   Ok(cnt)
// }

#[derive(BorshSerialize)]
struct VerifyTicketRequest<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub cnt_sui_address: &'a str,
}

#[derive(BorshSerialize)]
struct VerifyTicketMsg<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub ticket_owner_pubkey: &'a str,
  pub cnt_sui_address: &'a str,
  pub ticket_type_index: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VerificationResponse {
  pub event_id: String,
  pub code_challenge: String,
  pub ticket_owner_pubkey: String,
  pub cnt_sui_address: String,
  pub ticket_type_index: u8,
  pub server_sig: String,
}

fn create_mesage<'a>(msg: VerifyTicketMsg<'a>) -> Result<Vec<u8>> {
  let mut message: Vec<u8> = Vec::new();
  msg.serialize(&mut message)?;

  Ok(message)
}

fn sign_msg<'a>(signer_key: &str, msg: VerifyTicketMsg<'a>) -> Result<String> {
  let msg = create_mesage(msg)?;
  let sig = ed25519::sign(&msg, signer_key.as_bytes())?;

  Ok(sig.to_string())
}

fn redis_ticket_attended_key(event_id: &str, cnt_sui_address: &str) -> String {
  format!("{}:{}:attended", event_id, cnt_sui_address)
}

pub async fn verify_ticket(
  rpc_client: Arc<SuiClient>,
  pg_pool: &ConnectionPool,
  redis_pool: &redis::ConnectionPool,
  redlock: Arc<RedLock>,
  ticket_verifier_priv_key: String,
  event_id: &str,
  code_challenge: &str,
  cnt_sui_address: &str,
  ticket_owner_pubkey: &str,
  sig: &str,
) -> Result<VerificationResponse> {
  // 1. recover the signer
  let raw_message = VerifyTicketRequest {
    event_id: &event_id,
    code_challenge: &code_challenge,
    cnt_sui_address: &cnt_sui_address,
  };

  let mut message: Vec<u8> = Vec::new();
  raw_message.serialize(&mut message).unwrap();

  if let Ok(_) = ed25519::verify(&message, ticket_owner_pubkey.as_bytes(), &sig) {
    // 2. Load the ticket type index for the given cnt
    let mut postgres = pg_pool.connection().await?;
    let cnt = postgres.read_cnt(cnt_sui_address.to_string()).await?;
    let ticket_type_index = cnt.get(0).context("CNT not found")?.ticket_type_index as u8;

    // 3. check that signer is the owner of the given cnt_sui_addressk
    // let cnt_object = get_cnt_object(
    //   Arc::clone(&rpc_client),
    //   cnt_sui_address,
    // ).await?;

    // if cnt_object.account_id == ticket_owner_pubkey {
    if true {
      let server_sig = sign_msg(&ticket_verifier_priv_key, VerifyTicketMsg {
        event_id: &event_id,
        code_challenge: &code_challenge,
        ticket_owner_pubkey: &ticket_owner_pubkey,
        cnt_sui_address: &cnt_sui_address,
        ticket_type_index,
      })?;
      let redis_key = redis_ticket_attended_key(event_id, cnt_sui_address);

      let lock = redlock.lock(
        redis_key.as_bytes(),
        Duration::seconds(5).num_milliseconds() as usize,
      ).await?;
      let mut redis = redis_pool.connection().await?;

      // If key exists, it means someone has already attended this event
      if let Ok(_) = redis.get(&redis_key).await {
        return Err(Error::TicketVerificationError)?
      }

      redis.set(&redis_key, &"1".to_owned()).await?;
      redlock.unlock(lock).await;

      Ok(VerificationResponse {
        event_id: event_id.to_string(),
        code_challenge: code_challenge.to_string(),
        ticket_owner_pubkey: ticket_owner_pubkey.to_string(),
        cnt_sui_address: cnt_sui_address.to_string(),
        ticket_type_index,
        server_sig,
      })
    } else {
      return Err(Error::TicketVerificationError)?
    }
  } else {
    return Err(Error::TicketVerificationError)?
  }
}

pub fn validate_verification_result(
  verification_result: VerificationResponse,
  ticket_verifier_pub_key: String,
) -> Result<()> {
  let VerificationResponse {
    event_id,
    code_challenge,
    ticket_owner_pubkey,
    cnt_sui_address,
    ticket_type_index,
    server_sig,
  } = verification_result;

  let msg = create_mesage(VerifyTicketMsg {
    event_id: &event_id,
    code_challenge: &code_challenge,
    ticket_owner_pubkey: &ticket_owner_pubkey,
    cnt_sui_address: &cnt_sui_address,
    ticket_type_index,
  })?;

  ed25519::verify(&msg, ticket_verifier_pub_key.as_bytes(), &server_sig)?;

  Ok(())
}
