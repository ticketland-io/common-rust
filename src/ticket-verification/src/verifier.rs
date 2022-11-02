use std::{
  sync::Arc,
  str::FromStr,
};
use actix::prelude::*;
use eyre::{Result};
use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize};
use solana_sdk::{
  pubkey::Pubkey,
  signer::keypair::Keypair,
  signature::Signer,
  signature::Signature,
  keccak::hashv,
};
use common_data::{
  helpers::{send_read},
  models::ticket::Ticket,
  repositories::ticket::{read_ticket_by_ticket_metadata},
};
use ticketland_core::actor::neo4j::Neo4jActor;
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

fn sign_msg<'a>(signer_key: &str, msg: VerifyTicketMsg<'a>) -> String {
  let signer = Keypair::from_base58_string(signer_key);
  let mut message: Vec<u8> = Vec::new();
  msg.serialize(&mut message).unwrap();
  let message_hash = &hashv(&[&message]).0;

  bs58::encode(signer.sign_message(message_hash)).into_string()
}

pub async fn verify_ticket(
  rpc_client: Arc<RpcClient>,
  neo4j: Arc<Addr<Neo4jActor>>,
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
    let (query, db_query_params) = read_ticket_by_ticket_metadata(ticket_metadata.to_string());
    let ticket = send_read(neo4j, query, db_query_params)
    .await
    .map(TryInto::<Ticket>::try_into)??;
    let ticket_type_index = ticket.ticket_type_index;
    

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
      });

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
  ticket_verifier_priv_key: String,
) -> Result<()> {
  let VerificationResponse {
    event_id,
    code_challenge,
    ticket_owner_pubkey,
    ticket_metadata,
    ticket_type_index,
    server_sig,
  } = verification_result;
  
  let local_sig = sign_msg(&ticket_verifier_priv_key, VerifyTicketMsg {
    event_id: &event_id,
    code_challenge: &code_challenge,
    ticket_owner_pubkey: &ticket_owner_pubkey,
    ticket_metadata: &ticket_metadata,
    ticket_type_index,
  });

  if local_sig != server_sig {
    return Err(Error::InvalidVerificationResult)?
  }

  Ok(())
}
