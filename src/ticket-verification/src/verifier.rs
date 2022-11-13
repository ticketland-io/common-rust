use std::{
  sync::{Arc, Mutex},
  str::FromStr,
};
use chrono::Duration;
use eyre::Result;
use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize};
use solana_sdk::{
  pubkey::Pubkey,
  signature::Signature,
  keccak::hashv,
};
// use common_data::{
//   helpers::{send_read},
//   models::ticket::Ticket,
//   repositories::ticket::{read_ticket_by_ticket_metadata},
// };
use ticketland_crypto::asymetric::ed25519;
use ticketland_core::{services::{redis::Redis, redlock::RedLock}};
use ticketland_data::connection::PostgresConnection;
use program_artifacts::ticket_nft::account_data::TicketMetadata;
use solana_web3_rust::rpc_client::RpcClient;
use crate::error::Error;

#[derive(BorshSerialize)]
struct VerifyTicketRequest<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub ticket_metadata: &'a str,
}

#[derive(BorshSerialize)]
struct VerifyTicketMsg<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub ticket_owner_pubkey: &'a str,
  pub ticket_metadata: &'a str,
  pub ticket_type_index: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VerificationResponse {
  pub event_id: String,
  pub code_challenge: String,
  pub ticket_owner_pubkey: String,
  pub ticket_metadata: String,
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

fn redis_ticket_attended_key(event_id: &str, ticket_metadata: &str) -> String {
  format!("{}:{}:attended", event_id, ticket_metadata)
}

pub async fn verify_ticket(
  rpc_client: Arc<RpcClient>,
  postgres: Arc<Mutex<PostgresConnection>>,
  redis: Arc<Mutex<Redis>>,
  redlock: Arc<RedLock>,
  ticket_verifier_priv_key: String,
  event_id: &str,
  code_challenge: &str,
  ticket_metadata: &str,
  ticket_owner_pubkey: &str,
  sig: &str,
) -> Result<VerificationResponse> {
  // 1. recover the signer
  let raw_message = VerifyTicketRequest {
    event_id: &event_id,
    code_challenge: &code_challenge,
    ticket_metadata: &ticket_metadata,
  };

  let mut message: Vec<u8> = Vec::new();
  raw_message.serialize(&mut message).unwrap();
  let message_hash = &hashv(&[&message]).0;

  let sig = Signature::from_str(&sig).unwrap();
  let ticket_owner = Pubkey::from_str(&ticket_owner_pubkey).unwrap();

  if sig.verify(&ticket_owner.to_bytes(), message_hash) {
    // 2. Load the ticket type index for the given ticket
    let mut postgres = postgres.lock().unwrap();
    let ticket = postgres.read_ticket_by_ticket_metadata(ticket_metadata.to_string()).await?;
    let ticket_type_index = ticket.ticket_type_index as u8;
    

    // 3. check that signer is the owner of the given ticket_metadata 
    let ticket_metadata_account = rpc_client.get_anchor_account_data::<TicketMetadata>(
      &Pubkey::from_str(&ticket_metadata)?
    ).await?;

    if ticket_metadata_account.owner == ticket_owner {
      let server_sig = sign_msg(&ticket_verifier_priv_key, VerifyTicketMsg {
        event_id: &event_id,
        code_challenge: &code_challenge,
        ticket_owner_pubkey: &ticket_owner_pubkey,
        ticket_metadata: &ticket_metadata,
        ticket_type_index,
      })?;
      let redis_key = redis_ticket_attended_key(event_id, ticket_metadata);

      let lock = redlock.lock(
        redis_key.as_bytes(),
        Duration::seconds(5).num_seconds() as usize,
      ).await?;
      let mut redis = redis.lock().unwrap();

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
        ticket_metadata: ticket_metadata.to_string(),
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
    ticket_metadata,
    ticket_type_index,
    server_sig,
  } = verification_result;
  
  let msg = create_mesage(VerifyTicketMsg {
    event_id: &event_id,
    code_challenge: &code_challenge,
    ticket_owner_pubkey: &ticket_owner_pubkey,
    ticket_metadata: &ticket_metadata,
    ticket_type_index,
  })?;

  ed25519::verify(&msg, ticket_verifier_pub_key.as_bytes(), &server_sig)?;

  Ok(())
}
