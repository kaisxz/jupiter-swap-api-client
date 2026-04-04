use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

use build_transaction::BuildTransactionResponse;
use execute_transaction::{ExecuteTransactionRequest, ExecuteTransactionResponse};
use order::{OrderRequest, OrderResponse};
use transaction_config::TransactionConfig;

pub mod build_transaction;
pub mod execute_transaction;
pub mod instruction;
pub mod order;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod transaction_config;

const API_PATH: &str = "swap/v2";

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to deserialize response: {error}")]
    DeserializeFailed {
        error: serde_json::Error,
        body: String,
    },

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Invalid API key header value: {0}")]
    InvalidApiKey(#[from] reqwest::header::InvalidHeaderValue),
}

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    client: Client,
    base_url: Url,
}

impl JupiterSwapApiClient {
    /// Creates a new client for the given base URL (e.g. `"https://api.jup.ag"`).
    pub fn new(base_url: &str, api_key: Option<&str>) -> Result<Self, ClientError> {
        let mut url = Url::parse(base_url)?;
        if !url.path().ends_with('/') {
            url.set_path(&format!("{}/", url.path()));
        }

        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = api_key {
            headers.insert("x-api-key", key.parse()?);
        }

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            base_url: url,
        })
    }

    pub async fn order(&self, request: &OrderRequest) -> Result<OrderResponse, ClientError> {
        let url = self.endpoint("order")?;
        let response = self.client.get(url).query(request).send().await?;
        deserialize_response(response).await
    }

    pub async fn build(
        &self,
        request: &TransactionConfig,
    ) -> Result<BuildTransactionResponse, ClientError> {
        let url = self.endpoint("build")?;
        let response = self.client.post(url).json(request).send().await?;
        deserialize_response(response).await
    }

    pub async fn execute_transaction(
        &self,
        request: &ExecuteTransactionRequest,
    ) -> Result<ExecuteTransactionResponse, ClientError> {
        let url = self.endpoint("execute")?;
        let response = self.client.post(url).json(request).send().await?;
        deserialize_response(response).await
    }

    fn endpoint(&self, path: &str) -> Result<Url, ClientError> {
        Ok(self.base_url.join(&format!("{API_PATH}/{path}"))?)
    }
}

async fn deserialize_response<T: DeserializeOwned>(response: Response) -> Result<T, ClientError> {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(ClientError::RequestFailed { status, body });
    }

    serde_json::from_str::<T>(&body).map_err(|error| {
        ClientError::DeserializeFailed { error, body }
    })
}
