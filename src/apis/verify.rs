use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Verify API endpoints
pub struct VerifyApi<'a> {
    client: &'a Client,
}

impl<'a> VerifyApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Verify bank account details
    pub async fn verify_account(&self, request: &VerifyAccountRequest) -> ApiResult<VerifyAccountResponse> {
        self.client.post("/verify/v1/account", request).await
    }

    /// Verify BSB number
    pub async fn verify_bsb(&self, bsb: &str) -> ApiResult<VerifyBsbResponse> {
        self.client.get(&format!("/verify/v1/bsb/{}", bsb)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountRequest {
    pub account_number: String,
    pub bsb: String,
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountResponse {
    pub valid: bool,
    pub account_name: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyBsbResponse {
    pub valid: bool,
    pub bank_name: Option<String>,
    pub branch_name: Option<String>,
    pub address: Option<String>,
}

