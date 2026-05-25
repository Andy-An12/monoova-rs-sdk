use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// M Account API endpoints
pub struct MAccountApi<'a> {
    client: &'a Client,
}

impl<'a> MAccountApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List M Accounts (as issuer)
    pub async fn list(&self) -> ApiResult<MAccountListResponse> {
        self.client.get("/mAccount/v1/listAsIssuer").await
    }

    /// Get M Account financials (balance)
    pub async fn get_financials(&self, account_number: &str) -> ApiResult<MAccountFinancialsResponse> {
        self.client
            .get(&format!("/mAccount/v1/financials/{}", account_number))
            .await
    }

    /// Get M Account details
    pub async fn get(&self, account_number: &str) -> ApiResult<MAccountDetailsResponse> {
        self.client
            .get(&format!("/mAccount/v1/get/{}", account_number))
            .await
    }

    /// Get M Account transactions
    pub async fn get_transactions(&self, request: &MAccountTransactionsRequest) -> ApiResult<MAccountTransactionsResponse> {
        self.client.post("/mAccount/v1/transactions", request).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountListResponse {
    #[serde(rename = "mAccounts")]
    pub m_accounts: Option<Vec<serde_json::Value>>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub duration_ms: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountFinancialsResponse {
    pub account_number: Option<String>,
    pub available_balance: Option<f64>,
    pub pending_balance: Option<f64>,
    pub currency: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountDetailsResponse {
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    pub bsb: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountTransactionsRequest {
    pub account_number: String,
    pub frequency: String, // "Daily", "Weekly", "Monthly", "Custom"
    pub descending: Option<bool>,
    pub use_time: Option<bool>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountTransactionsResponse {
    pub items: Option<Vec<MAccountTransaction>>,
    pub closing_balance: Option<f64>,
    pub total_credits: Option<f64>,
    pub total_debits: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAccountTransaction {
    pub date_time: Option<String>,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
    pub running_balance: Option<f64>,
    pub unique_reference: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

