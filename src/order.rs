//! `/swap/v2/order` request and response types.
//!
//! Depends on `serde`, `rust_decimal` (`priceImpactPct`), `solana_sdk::pubkey`,
//! and [`crate::route_plan_with_metadata::SwapInfo`]. Optional response fields
//! are frequently omitted; they deserialize as `None`.

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
    /// Available routers: `metis`, `jupiterz`, `dflow`, `okx`.
    /// `iris` is still accepted as an alias for `metis`. The response `router` is `metis`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_routers: Option<String>,
    /// Comma-separated list of DEXes to exclude from the Metis router.
    /// This only affects the Metis router, not other routers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_dexes: Option<String>,
    /// Highest Solana transaction version this client can sign: `0` or `1`.
    /// This is a ceiling. v1 is returned only when Metis wins and the order is not gasless.
    /// Read [`OrderResponse::transaction_version`] for the version that was actually built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    pub swap_info: SwapInfo,
    /// Share of the swap routed through this hop, in percent.
    ///
    /// Split routes are fractional: `21.6` means 21.6%. [`Self::bps`] is the
    /// integer weight of the same hop (`2160`, and `10000` = 100%).
    pub percent: f64,
    /// Basis points of the swap routed through this hop. `10000` is 100%.
    pub bps: u16,
    /// USD value flowing through this hop. Omitted on some routers (JupiterZ).
    #[serde(default)]
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
    /// `"ultra"` when no optional parameters were sent, `"manual"` otherwise.
    #[serde(default)]
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
    /// Price impact in percentage points (`-0.1` means -0.1%). Prefer this over `price_impact_pct`.
    #[serde(default)]
    pub price_impact: Option<f64>,
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    pub route_plan: Vec<RoutePlanStep>,
    #[serde(with = "field_as_string")]
    pub fee_mint: Pubkey,
    pub fee_bps: u16,
    #[serde(default)]
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
    /// Winning router: `metis`, `jupiterz`, `dflow`, or `okx`.
    #[serde(default)]
    pub router: Option<String>,
    /// Base64-encoded unsigned transaction.
    /// `None` when `taker` was omitted. Empty string when a taker was set but the transaction could not be built.
    #[serde(default)]
    pub transaction: Option<String>,
    /// Solana message version of `transaction`: `0` or `1`.
    /// Absent on quote-only responses. With `maxSupportedTransactionVersion=1` this is `1`
    /// only when Metis wins and the order is not gasless.
    #[serde(default)]
    pub transaction_version: Option<u8>,
    pub gasless: bool,
    /// `true` when the winning router guarantees the quoted price (JupiterZ RFQ).
    #[serde(default)]
    pub guaranteed_price: bool,
    /// Live `/order` flag. Deserializes as `false` when the field is absent.
    #[serde(default)]
    pub jit_optimized: bool,
    pub request_id: String,
    #[serde(default)]
    pub total_time: Option<u64>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub taker: Option<Pubkey>,
    #[serde(default)]
    pub last_valid_block_height: Option<String>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub referral_account: Option<Pubkey>,
    #[serde(default)]
    pub quote_id: Option<String>,
    #[serde(with = "option_field_as_string")]
    #[serde(default)]
    pub maker: Option<Pubkey>,
    #[serde(default)]
    pub expire_at: Option<String>,
    #[serde(default)]
    pub error_code: Option<i32>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::{OrderResponse, RoutePlanStep};

    const SWAP_INFO: &str = r#"{
        "ammKey": "7yD7gfXv49SFxhx9uTc11dEd7Nj98rGWeYMdZJgheZbG",
        "label": "Raydium CLMM",
        "inputMint": "So11111111111111111111111111111111111111112",
        "outputMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "inAmount": "1368447",
        "outAmount": "137789"
    }"#;

    #[test]
    fn route_step_accepts_fractional_percent_without_usd_value() {
        let json = format!(r#"{{"percent":21.6,"bps":2160,"swapInfo":{SWAP_INFO}}}"#);
        let step: RoutePlanStep = serde_json::from_str(&json).unwrap();
        assert_eq!(step.percent, 21.6);
        assert_eq!(step.bps, 2160);
        assert!(step.usd_value.is_none());
    }

    #[test]
    fn order_response_matches_live_quote() {
        // Captured from GET /swap/v2/order (SOL -> USDC, no taker).
        // Optional keys such as lastValidBlockHeight, quoteId, and error are absent.
        let json = r#"{
            "swapType":"aggregator",
            "inAmount":"1000000000",
            "outAmount":"100579957",
            "otherAmountThreshold":"100579957",
            "swapMode":"ExactIn",
            "slippageBps":0,
            "priceImpactPct":"-0.0002663431267852168",
            "routePlan":[{
                "percent":100,
                "bps":10000,
                "usdValue":100.51980537756539,
                "swapInfo":{
                    "ammKey":"7yD7gfXv49SFxhx9uTc11dEd7Nj98rGWeYMdZJgheZbG",
                    "label":"Kipseli",
                    "inputMint":"So11111111111111111111111111111111111111112",
                    "outputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                    "inAmount":"1000000000",
                    "outAmount":"100600077"
                }
            }],
            "feeMint":"So11111111111111111111111111111111111111112",
            "feeBps":2,
            "platformFee":{"feeBps":2,"feeMint":"So11111111111111111111111111111111111111112"},
            "signatureFeeLamports":0,
            "signatureFeePayer":null,
            "prioritizationFeeLamports":0,
            "prioritizationFeePayer":null,
            "rentFeeLamports":0,
            "rentFeePayer":null,
            "transaction":null,
            "gasless":false,
            "jitOptimized":false,
            "taker":null,
            "inputMint":"So11111111111111111111111111111111111111112",
            "outputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            "router":"metis",
            "guaranteedPrice":false,
            "requestId":"01a17987-9591-775c-95a8-87547951b6d7",
            "inUsdValue":100.54658526947364,
            "outUsdValue":100.51980537756539,
            "swapUsdValue":100.51980537756539,
            "priceImpact":-0.02663431267852168,
            "mode":"ultra",
            "totalTime":53
        }"#;

        let order: OrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order.route_plan[0].percent, 100.0);
        assert_eq!(order.route_plan[0].bps, 10_000);
        assert!(order.platform_fee.unwrap().amount.is_none());
        assert!(!order.guaranteed_price);
        assert!(!order.jit_optimized);
        assert!(order.transaction_version.is_none());
        assert!(order.last_valid_block_height.is_none());
        assert!(order.quote_id.is_none());
        assert!(order.error_code.is_none());
        assert_eq!(order.router.as_deref(), Some("metis"));

        let split = json
            .replace("\"percent\":100", "\"percent\":21.6")
            .replace("\"bps\":10000", "\"bps\":2160");
        let split: OrderResponse = serde_json::from_str(&split).unwrap();
        assert_eq!(split.route_plan[0].percent, 21.6);
        assert_eq!(split.route_plan[0].bps, 2160);
    }
}
