use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Receivables API endpoints
pub struct ReceivablesApi<'a> {
    client: &'a Client,
}

impl<'a> ReceivablesApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create account
    pub async fn create_account(&self, request: &CreateAccountRequest) -> ApiResult<CreateAccountResponse> {
        self.client.post("/receivables/v1/create", request).await
    }

    /// Get account status by client ID
    pub async fn get_account_status(&self, client_id: &str) -> ApiResult<AccountStatusResponse> {
        self.client
            .get(&format!("/receivables/v1/statusByClientID/{}", client_id))
            .await
    }

    /// Set account status
    pub async fn set_account_status(&self, request: &SetAccountStatusRequest) -> ApiResult<SetAccountStatusResponse> {
        self.client.post("/receivables/v1/status", request).await
    }

    /// Register Pay ID
    pub async fn register_payid(&self, request: &RegisterPayIdRequest) -> ApiResult<RegisterPayIdResponse> {
        self.client.post("/receivables/v1/payid/registerpayid", request).await
    }

    /// Get Pay ID status
    pub async fn get_payid_status(&self, request: &PayIdEnquiryRequest) -> ApiResult<PayIdStatusResponse> {
        self.client.post("/receivables/v1/payid/payIdEnquiry", request).await
    }

    /// Update Pay ID status
    pub async fn update_payid_status(&self, request: &UpdatePayIdStatusRequest) -> ApiResult<UpdatePayIdStatusResponse> {
        self.client.post("/receivables/v1/payid/updatePayIdStatus", request).await
    }

    /// Process refund
    pub async fn refund(&self, request: &RefundRequest) -> ApiResult<RefundResponse> {
        self.client.post("/receivables/v2/refund", request).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatusResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAccountStatusRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAccountStatusResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayIdRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayIdResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdEnquiryRequest {
    pub pay_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdStatusResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayIdStatusRequest {
    pub bank_account_number: String,
    pub bsb: String,
    pub pay_id: String,
    pub status: String, // "Enable" or "Disable"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayIdStatusResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub original_transaction_id: Option<String>,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

