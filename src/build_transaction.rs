//! Data structures for the Jupiter Swap API build endpoint (`POST /swap/v2/build`).
//!
//! The build endpoint takes a quote and returns decomposed Solana instructions
//! that the caller can assemble into a transaction for signing.

use std::collections::HashMap;

use crate::instruction::Instruction;
use crate::route_plan_with_metadata::SwapInfo;
use crate::serde_helpers::field_as_string;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// ── Response ────────────────────────────────────────────────────────────────

/// Successful response from `POST /swap/v2/build`.
///
/// Contains the decomposed instructions for a swap transaction:
/// compute budget, setup, the swap itself, cleanup, and any extras.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildTransactionResponse {
    // ── Quote summary ───────────────────────────────────────────────────
    /// The mint address of the input token.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint address of the output token.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// Input amount in native token units (before decimals).
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// Expected output amount in native token units.
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Minimum output (or maximum input) depending on `swap_mode`.
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    /// `"ExactIn"` or `"ExactOut"`.
    pub swap_mode: String,
    /// Maximum slippage tolerance in basis points.
    pub slippage_bps: u16,

    // ── Route ───────────────────────────────────────────────────────────
    /// The route the swap will take through one or more AMMs.
    pub route_plan: Vec<BuildRoutePlanStep>,

    // ── Instructions ────────────────────────────────────────────────────
    /// Instructions that set the compute-unit budget for the transaction.
    #[serde(default)]
    pub compute_budget_instructions: Vec<Instruction>,
    /// Instructions that must run before the swap (e.g. creating token accounts).
    #[serde(default)]
    pub setup_instructions: Vec<Instruction>,
    /// The main swap instruction.
    pub swap_instruction: Instruction,
    /// Optional instruction that runs after the swap (e.g. closing WSOL accounts).
    pub cleanup_instruction: Option<Instruction>,
    /// Any additional instructions not covered by the above categories.
    #[serde(default)]
    pub other_instructions: Vec<Instruction>,

    // ── Lookup tables & blockhash ───────────────────────────────────────
    /// Address-lookup-table addresses mapped to the accounts they contain.
    /// Used for versioned transactions to reduce transaction size.
    #[serde(default)]
    pub addresses_by_lookup_table_address: HashMap<String, Vec<String>>,
    /// Recent blockhash and its expiry height, if provided by the API.
    pub blockhash_with_metadata: Option<BlockhashWithMetadata>,
}

// ── Supporting types ────────────────────────────────────────────────────────

/// A recent blockhash together with the last block height at which it is valid.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockhashWithMetadata {
    /// Raw blockhash bytes.
    pub blockhash: Vec<u8>,
    /// The last block height at which `blockhash` can be used.
    pub last_valid_block_height: u64,
}

/// A single step in the build response's route plan.
///
/// Extends the base swap info with routing weight (`percent`),
/// fee contribution (`bps`), and an optional USD valuation.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildRoutePlanStep {
    /// Details about the AMM and the token pair for this hop.
    pub swap_info: SwapInfo,
    /// Percentage of the total input routed through this hop.
    pub percent: u8,
    /// Fee contribution of this hop in basis points.
    pub bps: u16,
    /// Estimated USD value flowing through this hop.
    pub usd_value: Option<f64>,
}
