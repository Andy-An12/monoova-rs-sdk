use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Tools API endpoints
pub struct ToolsApi<'a> {
    client: &'a Client,
}

impl<'a> ToolsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Validate BSB number
    pub async fn validate_bsb(&self, bsb: &str) -> ApiResult<BsbValidationResponse> {
        self.client.get(&format!("/tools/v1/validate/bsb/{}", bsb)).await
    }

    /// Validate account number
    pub async fn validate_account(&self, account_number: &str, bsb: &str) -> ApiResult<AccountValidationResponse> {
        self.client
            .get(&format!("/tools/v1/validate/account/{}/{}", account_number, bsb))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsbValidationResponse {
    pub valid: bool,
    pub bank_name: Option<String>,
    pub branch_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountValidationResponse {
    pub valid: bool,
    pub message: Option<String>,
}

