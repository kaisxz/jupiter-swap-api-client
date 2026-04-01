//! Data structures for the Jupiter Swap API execute endpoint (`POST /swap/v2/execute`).
//!
//! After building and signing a transaction via `/swap/v2/build`, submit it
//! here for managed landing with automatic retry and confirmation.

use crate::serde_helpers::field_as_string;
use crate::serde_helpers::option_field_as_string;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// ── Request ─────────────────────────────────────────────────────────────────

/// Request body for `POST /swap/v2/execute`.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteTransactionRequest {
    /// Base64-encoded signed transaction obtained from `/swap/v2/build`.
    pub signed_transaction: String,
    /// The `requestId` returned by the `/order` response.
    pub request_id: String,
    /// Optional last valid block height for nonce-based validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_valid_block_height: Option<String>,
}

// ── Response ────────────────────────────────────────────────────────────────

/// Response from `POST /swap/v2/execute`.
///
/// # Error codes
///
/// | Range | Source     | Codes |
/// |-------|-----------|-------|
/// | 0     | Success   | — |
/// | -1‥  | Ultra     | -1 missing cached order, -2 invalid signed tx, -3 invalid message bytes |
/// | -1000‥| Aggregator| -1000 failed to land, -1001 unknown, -1002 invalid tx, -1003 not fully signed, -1004 invalid block height |
/// | -2000‥| RFQ       | -2000 failed to land, -2001 unknown, -2002 invalid payload, -2003 quote expired, -2004 swap rejected |
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteTransactionResponse {
    /// `"Success"` or `"Failed"`.
    pub status: String,
    /// The on-chain transaction signature, if available.
    pub signature: Option<String>,
    /// The slot in which the transaction was confirmed.
    pub slot: Option<String>,
    /// Human-readable error message (present when `status` is `"Failed"`).
    pub error: Option<String>,
    /// Numeric error code. `0` means success; see table above for failure codes.
    pub code: i32,

    // ── Amounts ─────────────────────────────────────────────────────────
    /// Total input token amount before fees.
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub total_input_amount: Option<u64>,
    /// Total output token amount after fees.
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub total_output_amount: Option<u64>,
    /// Actual input amount consumed by the swap.
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub input_amount_result: Option<u64>,
    /// Actual output amount received from the swap.
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub output_amount_result: Option<u64>,

    // ── Events ──────────────────────────────────────────────────────────
    /// Per-hop swap events with individual input/output amounts.
    #[serde(default)]
    pub swap_events: Option<Vec<SwapEvent>>,
}

// ── Supporting types ────────────────────────────────────────────────────────

/// A single swap event describing one hop in the route.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    /// Mint of the token going into this hop.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// Amount of the input token for this hop.
    #[serde(with = "field_as_string")]
    pub input_amount: u64,
    /// Mint of the token coming out of this hop.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// Amount of the output token for this hop.
    #[serde(with = "field_as_string")]
    pub output_amount: u64,
}
