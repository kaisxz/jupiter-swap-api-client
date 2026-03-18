//! Order and Execute data structures for the Jupiter Ultra API
//! (`/ultra/v1/order` and `/ultra/v1/execute` endpoints)
//!

use crate::route_plan_with_metadata::SwapInfo;
use crate::serde_helpers::field_as_string;
use crate::serde_helpers::option_field_as_string;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount of input token to swap, in native token units (before decimals).
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The user's wallet address.
    #[serde(with = "field_as_string")]
    pub taker: Pubkey,
    /// The referral account address for fee sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub referral_account: Option<Pubkey>,
    /// The referral fee in basis points (bps).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_fee: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    pub swap_info: SwapInfo,
    pub percent: u8,
    pub bps: u16,
    pub usd_value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderPlatformFee {
    pub fee_bps: u16,
    #[serde(with = "field_as_string")]
    pub fee_mint: Pubkey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub mode: Option<String>,
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    pub in_usd_value: f64,
    pub out_usd_value: f64,
    pub swap_usd_value: f64,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub price_impact_pct: Decimal,
    pub price_impact: Option<f64>,
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    pub route_plan: Vec<RoutePlanStep>,
    #[serde(with = "field_as_string")]
    pub fee_mint: Pubkey,
    pub fee_bps: u16,
    pub platform_fee: Option<OrderPlatformFee>,
    pub signature_fee_lamports: u64,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub signature_fee_payer: Option<Pubkey>,
    pub prioritization_fee_lamports: u64,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub prioritization_fee_payer: Option<Pubkey>,
    pub rent_fee_lamports: u64,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub rent_fee_payer: Option<Pubkey>,
    pub swap_type: String,
    pub router: Option<String>,
    /// Base64 encoded unsigned transaction. Only present when `taker` was provided.
    pub transaction: Option<String>,
    pub gasless: bool,
    pub request_id: String,
    pub total_time: Option<u64>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub taker: Option<Pubkey>,
    pub last_valid_block_height: Option<String>,
}

/// Request body for `POST /ultra/v1/execute`.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
    /// The signed and serialized base64-encoded transaction.
    pub signed_transaction: String,
    /// The request ID from the order response.
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub input_amount: u64,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_amount: u64,
}

/// Response from `POST /ultra/v1/execute`.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub status: String,
    pub signature: Option<String>,
    pub slot: Option<String>,
    pub code: i32,
    pub error: Option<String>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub input_amount_result: Option<u64>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub output_amount_result: Option<u64>,
    #[serde(default)]
    pub swap_events: Option<Vec<SwapEvent>>,
}
