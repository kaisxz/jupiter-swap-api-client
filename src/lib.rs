use order::{ExecuteRequest, ExecuteResponse, OrderRequest, OrderResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;

pub mod order;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] reqwest::Error),
}

async fn check_is_success(response: Response) -> Result<Response, ClientError> {
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(ClientError::RequestFailed { status, body });
    }
    Ok(response)
}

async fn check_status_code_and_deserialize<T: DeserializeOwned>(
    response: Response,
) -> Result<T, ClientError> {
    let response = check_is_success(response).await?;
    response
        .json::<T>()
        .await
        .map_err(ClientError::DeserializationError)
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String, api_key: Option<String>) -> Self {
        Self { base_path, api_key }
    }

    fn build_client(&self) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(ref key) = self.api_key {
            headers.insert("x-api-key", key.parse().expect("Invalid API key header value"));
        }
        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client")
    }

    pub async fn order(
        &self,
        order_request: &OrderRequest,
    ) -> Result<OrderResponse, ClientError> {
        let url = format!("{}/ultra/v1/order", self.base_path);
        let response = self
            .build_client()
            .get(url)
            .query(order_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn execute(
        &self,
        execute_request: &ExecuteRequest,
    ) -> Result<ExecuteResponse, ClientError> {
        let url = format!("{}/ultra/v1/execute", self.base_path);
        let response = self
            .build_client()
            .post(url)
            .json(execute_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }
}
