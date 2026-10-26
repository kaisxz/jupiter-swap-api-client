//! Data structures for the Jupiter Swap API build endpoint (`GET /swap/v2/build`).
//!
//! Depends on `serde`, `rust_decimal`, `solana_sdk::pubkey`, [`crate::instruction::Instruction`],
//! and [`crate::route_plan_with_metadata::SwapInfo`]. The endpoint is Metis-only:
//! it returns a quote plus raw instructions. `/execute` does not accept these transactions.

use std::collections::HashMap;

use crate::instruction::Instruction;
use crate::route_plan_with_metadata::SwapInfo;
use crate::serde_helpers::field_as_string;
use crate::serde_helpers::null_as_default;
use crate::serde_helpers::option_field_as_string;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
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
    /// Price impact as a decimal ratio string (`"0.001"` = 0.1%).
    #[serde(default, with = "option_field_as_string")]
    pub price_impact_pct: Option<Decimal>,

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
    #[serde(default)]
    pub cleanup_instruction: Option<Instruction>,
    /// Any additional instructions not covered by the above categories.
    #[serde(default)]
    pub other_instructions: Vec<Instruction>,
    /// SOL tip transfer. Present when `tipAmount` was set on the request.
    #[serde(default)]
    pub tip_instruction: Option<Instruction>,

    // ── Lookup tables & blockhash ───────────────────────────────────────
    /// Address-lookup-table addresses mapped to the accounts they contain.
    /// Empty when `transactionVersion` is `1` (accounts are inlined). The API also sends `null`.
    #[serde(default, deserialize_with = "null_as_default")]
    pub addresses_by_lookup_table_address: HashMap<String, Vec<String>>,
    /// Solana message version these instructions were built for: `0` or `1`.
    #[serde(default)]
    pub transaction_version: Option<u8>,
    /// Compute unit price in micro-lamports. Present when `transactionVersion` is `1`,
    /// because v1 carries the compute budget on the message instead of instructions.
    #[serde(default, with = "option_field_as_string")]
    pub compute_unit_price: Option<u64>,
    /// Recent blockhash and its expiry height, if provided by the API.
    #[serde(default)]
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
    /// When the blockhash was fetched. The API uses snake_case keys inside this object.
    #[serde(default)]
    pub fetched_at: Option<BlockhashFetchedAt>,
}

/// Unix timestamp split into seconds and nanoseconds, as returned on `blockhashWithMetadata.fetchedAt`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlockhashFetchedAt {
    pub secs_since_epoch: u64,
    pub nanos_since_epoch: u64,
}

/// A single step in the build response's route plan.
///
/// `percent` may be fractional. `bps` is the integer weight (`10000` = 100%).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildRoutePlanStep {
    /// Details about the AMM and the token pair for this hop.
    pub swap_info: SwapInfo,
    /// Share of the swap routed through this hop, in percent.
    /// Split routes are fractional (`21.6` means 21.6%).
    pub percent: f64,
    /// Basis points of the swap routed through this hop. `10000` is 100%.
    pub bps: u16,
    /// Estimated USD value flowing through this hop. Often omitted.
    #[serde(default)]
    pub usd_value: Option<f64>,
}

// ── Request ─────────────────────────────────────────────────────────────────

/// Query parameters for `GET /swap/v2/build`.
///
/// Routing is Metis only. `taker` is required. Slippage defaults to 50 bps on the API
/// when [`Self::slippage_bps`] is omitted.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuildRequest {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// Amount in the smallest unit of the input token.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// Wallet that signs the swap.
    #[serde(with = "field_as_string")]
    pub taker: Pubkey,
    /// Slippage in basis points, or [`BuildSlippageBps::Rtse`] for the real-time estimator.
    /// The API default is 50 when this is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<BuildSlippageBps>,
    /// `"fast"` trades quote optimality for lower latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<BuildMode>,
    /// Comma-separated DEX labels to allow. Mutually exclusive with `exclude_dexes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexes: Option<String>,
    /// Comma-separated DEX labels to exclude. Mutually exclusive with `dexes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_dexes: Option<String>,
    /// Platform fee in basis points. Requires `fee_account` when greater than zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_bps: Option<u16>,
    /// Token account that receives the platform fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub fee_account: Option<Pubkey>,
    /// Maximum accounts in the route. The API default is 64. Range 1–64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_accounts: Option<u8>,
    /// Account that pays transaction fees and rent. Defaults to `taker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub payer: Option<Pubkey>,
    /// Wrap and unwrap SOL. The API default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_and_unwrap_sol: Option<bool>,
    /// SPL token account that receives the output. Mutually exclusive with `native_destination_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub destination_token_account: Option<Pubkey>,
    /// Native SOL account that receives the output. Mutually exclusive with `destination_token_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub native_destination_account: Option<Pubkey>,
    /// Slots until the returned blockhash expires. The API default is 150. Range 1–300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash_slots_to_expiry: Option<u16>,
    /// SOL tip in lamports. Adds `tipInstruction` for landing via `/submit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_price_percentile: Option<ComputeUnitPricePercentile>,
    /// Exclude DEXes that cannot land inside a Jito bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_jito_bundle: Option<bool>,
    /// Solana transaction version to build: `0` (default) or `1`.
    /// `1` forces v1: empty compute-budget instructions, no lookup tables, and `compute_unit_price` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_version: Option<u8>,
}

/// Slippage for `/build`. A fixed bps value, or `rtse` for the real-time estimator.
#[derive(Debug, Clone)]
pub enum BuildSlippageBps {
    Bps(u16),
    Rtse,
}

impl Serialize for BuildSlippageBps {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bps(bps) => serializer.serialize_u16(*bps),
            Self::Rtse => serializer.serialize_str("rtse"),
        }
    }
}

/// Quoting mode for `/build`.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BuildMode {
    Fast,
}

/// Priority-fee percentile used to price the compute-unit instruction.
#[derive(Debug, Clone)]
pub enum ComputeUnitPricePercentile {
    /// 25th percentile.
    Medium,
    /// 50th percentile.
    High,
    /// 75th percentile.
    VeryHigh,
    /// Explicit percentile in basis points, 0–10000.
    Bps(u16),
}

impl Serialize for ComputeUnitPricePercentile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Medium => serializer.serialize_str("medium"),
            Self::High => serializer.serialize_str("high"),
            Self::VeryHigh => serializer.serialize_str("veryHigh"),
            Self::Bps(bps) => serializer.serialize_u16(*bps),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BuildTransactionResponse;

    #[test]
    fn build_response_accepts_fractional_percent_and_v1_fields() {
        let json = r#"{
            "inputMint": "So11111111111111111111111111111111111111112",
            "outputMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            "inAmount": "1000",
            "outAmount": "100",
            "otherAmountThreshold": "99",
            "swapMode": "ExactIn",
            "slippageBps": 50,
            "priceImpactPct": "0",
            "routePlan": [{
                "percent": 21.6,
                "bps": 2160,
                "swapInfo": {
                    "ammKey": "7yD7gfXv49SFxhx9uTc11dEd7Nj98rGWeYMdZJgheZbG",
                    "label": "Kipseli",
                    "inputMint": "So11111111111111111111111111111111111111112",
                    "outputMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                    "inAmount": "1000",
                    "outAmount": "100"
                }
            }],
            "computeBudgetInstructions": [],
            "setupInstructions": [],
            "swapInstruction": {
                "programId": "7yD7gfXv49SFxhx9uTc11dEd7Nj98rGWeYMdZJgheZbG",
                "accounts": [],
                "data": ""
            },
            "cleanupInstruction": null,
            "otherInstructions": [],
            "tipInstruction": null,
            "addressesByLookupTableAddress": null,
            "transactionVersion": 1,
            "computeUnitPrice": "7826",
            "blockhashWithMetadata": {
                "blockhash": [1, 2, 3],
                "lastValidBlockHeight": 10,
                "fetchedAt": {"secs_since_epoch": 1, "nanos_since_epoch": 2}
            }
        }"#;

        let response: BuildTransactionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.route_plan[0].percent, 21.6);
        assert_eq!(response.route_plan[0].bps, 2160);
        assert!(response.route_plan[0].usd_value.is_none());
        assert!(response.addresses_by_lookup_table_address.is_empty());
        assert_eq!(response.transaction_version, Some(1));
        assert_eq!(response.compute_unit_price, Some(7826));
        let fetched = response
            .blockhash_with_metadata
            .unwrap()
            .fetched_at
            .unwrap();
        assert_eq!(fetched.secs_since_epoch, 1);
        assert_eq!(fetched.nanos_since_epoch, 2);
    }
}
