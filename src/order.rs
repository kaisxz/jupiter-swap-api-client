use crate::route_plan_with_metadata::SwapInfo;
use crate::serde_helpers::field_as_string;
use crate::serde_helpers::option_field_as_string;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Swap mode for the order.
/// Currently only `ExactIn` is supported.
#[derive(Serialize, Debug, Clone, Default)]
pub enum SwapMode {
    #[default]
    ExactIn,
}

/// Fee cap strategy for broadcast fees.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BroadcastFeeType {
    /// Treats the fee as a maximum cap.
    MaxCap,
    /// Uses the exact fee amount.
    ExactFee,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    /// The mint address of the input token.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint address of the output token.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount to swap in the smallest unit of the input token.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The public key of the wallet that will sign the transaction.
    /// If not provided, the response will contain a quote but no transaction.
    /// Must be present if you intend to sign and execute the transaction via /execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub taker: Option<Pubkey>,
    /// The public key of the account that will receive the output tokens.
    /// Must differ from taker. Expects a wallet address, not a token account.
    /// Disables RFQ routing (JupiterZ).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub receiver: Option<Pubkey>,
    /// Swap mode. Currently only ExactIn is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_mode: Option<SwapMode>,
    /// Slippage tolerance in basis points (0–10000).
    /// If not set, Jupiter automatically determines an appropriate slippage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<u16>,
    /// Address of your referral account for the Jupiter referral project.
    /// Must be used together with `referral_fee`. Disables RFQ routing (JupiterZ).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub referral_account: Option<Pubkey>,
    /// Referral fee in basis points (50–255).
    /// Must be used together with `referral_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_fee: Option<u16>,
    /// The public key of an account that will cover gas-related fees
    /// (signature fees, priority fees, and rent) on behalf of the taker.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub payer: Option<Pubkey>,
    /// Priority fee in lamports.
    /// If not set, Jupiter automatically determines an appropriate priority fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee_lamports: Option<u64>,
    /// Jito MEV tip in lamports for faster block inclusion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jito_tip_lamports: Option<u64>,
    /// Fee cap strategy: `MaxCap` treats the fee as a maximum, `ExactFee` uses the exact amount.
    /// Ignored if neither `priority_fee_lamports` nor `jito_tip_lamports` are set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_fee_type: Option<BroadcastFeeType>,
    /// Comma-separated list of routers to exclude.
    /// Available routers: iris (Metis), jupiterz (JupiterZ), dflow, okx.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_routers: Option<String>,
    /// Comma-separated list of DEXes to exclude from the Metis router.
    /// This only affects the Metis router, not other routers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_dexes: Option<String>,
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
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub amount: Option<u64>,
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
    #[serde(with = "field_as_string")]
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
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub referral_account: Option<Pubkey>,
    pub quote_id: Option<String>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub maker: Option<Pubkey>,
    pub expire_at: Option<String>,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub error: Option<String>,
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
