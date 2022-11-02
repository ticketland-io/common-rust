use std::{
  sync::Arc,
  str::FromStr,
};
use eyre::{Result};
use borsh::BorshSerialize;
use solana_sdk::{
  pubkey::Pubkey,
  signer::keypair::Keypair,
  signature::Signer,
  signature::Signature,
  keccak::hashv,
};
use program_artifacts::ticket_nft::account_data::TicketMetadata;
use solana_web3_rust::rpc_client::RpcClient;
use crate::error::Error;

#[derive(BorshSerialize)]
struct VerifyTicketMsg<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub ticket_metadata: &'a str,
}

#[derive(BorshSerialize)]
struct VerifyTicketResult<'a> {
  pub event_id: &'a str,
  pub code_challenge: &'a str,
  pub ticket_owner_pubkey: &'a str,
  pub ticket_metadata: &'a str,
}

fn sign_msg<'a>(signer_key: &str, msg: VerifyTicketResult<'a>) -> String {
  let signer = Keypair::from_base58_string(signer_key);
  let mut message: Vec<u8> = Vec::new();
  msg.serialize(&mut message).unwrap();
  let message_hash = &hashv(&[&message]).0;

  bs58::encode(signer.sign_message(message_hash)).into_string()
}

pub async fn verify_ticket(
  rpc_client: Arc<RpcClient>,
  ticket_verifier_priv_key: String,
  event_id: &str,
  code_challenge: &str,
  ticket_metadata: &str,
  ticket_owner_pubkey: &str,
  sig: &str,
) -> Result<String> {
  // 1. recover the signer
  let raw_message = VerifyTicketMsg {
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
    // 2. check that signer is the owner of the given ticket_metadata 
    let ticket_metadata_account = rpc_client.get_anchor_account_data::<TicketMetadata>(
      &Pubkey::from_str(&ticket_metadata)?
    ).await?;

    if ticket_metadata_account.owner == ticket_owner {
      let sig = sign_msg(&ticket_verifier_priv_key, VerifyTicketResult {
        event_id: &event_id,
        code_challenge: &code_challenge,
        ticket_owner_pubkey: &ticket_owner_pubkey,
        ticket_metadata: &ticket_metadata,
      });

      Ok(sig)
    } else {
      return Err(Error::TicketVerificationError)?
    }
  } else {
    return Err(Error::TicketVerificationError)?
  }
}
