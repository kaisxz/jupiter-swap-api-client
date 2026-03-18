use std::env;

use jupiter_swap_api_client::{
    order::{ExecuteRequest, OrderRequest},
    JupiterSwapApiClient,
};
use solana_sdk::{pubkey, pubkey::Pubkey};

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[tokio::main]
async fn main() {
    let api_base_url =
        env::var("API_BASE_URL").unwrap_or("https://api.jup.ag".into());
    println!("Using base url: {}", api_base_url);

    let api_key = env::var("JUPITER_API_KEY").ok();
    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url, api_key);

    let order_request = OrderRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: NATIVE_MINT,
        taker: TEST_WALLET,
        ..OrderRequest::default()
    };

    // GET /ultra/v1/order
    let order_response = jupiter_swap_api_client
        .order(&order_request)
        .await
        .unwrap();
    println!("{order_response:#?}");

    // Sign the transaction (requires a real keypair in production)
    // let transaction = VersionedTransaction::deserialize(...);
    // transaction.sign([&keypair]);
    // let signed_transaction = base64::encode(transaction.serialize());

    // POST /ultra/v1/execute
    let _execute_response = jupiter_swap_api_client
        .execute(&ExecuteRequest {
            signed_transaction: "signed-base64-transaction".into(),
            request_id: order_response.request_id.clone(),
        })
        .await;
    println!("Execute response: {_execute_response:#?}");
}
