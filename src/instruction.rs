//! Shared Solana instruction types used across the Jupiter Swap API.
//!
//! These types represent the on-chain instruction format returned by
//! endpoints like `/swap/v2/build`.

use crate::serde_helpers::field_as_string;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// A single account referenced by a Solana instruction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountMeta {
    /// The public key of the account.
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    /// Whether the account is writable by the instruction.
    pub is_writable: bool,
    /// Whether the account must be a signer of the transaction.
    pub is_signer: bool,
}

/// A Solana instruction as returned by the Jupiter API.
///
/// Contains the target program, the accounts it operates on,
/// and the base64-encoded instruction data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    /// The program that will process this instruction.
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    /// Accounts required by the instruction.
    pub accounts: Vec<AccountMeta>,
    /// Base64-encoded instruction data.
    pub data: String,
}
