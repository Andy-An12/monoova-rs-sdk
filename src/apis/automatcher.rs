use crate::apis::utils;
use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Automatcher API endpoints
pub struct AutomatcherApi<'a> {
    client: &'a Client,
}

impl<'a> AutomatcherApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get automatcher bank account receivables
    pub async fn get_receivables(&self, params: &ReceivablesParams) -> ApiResult<ReceivablesResponse> {
        let mut query_parts = Vec::new();
        query_parts.push(utils::build_query_string(&[
            ("from_date", params.from_date.clone()),
            ("to_date", params.to_date.clone()),
        ]));
        query_parts.push(utils::build_query_string_with_numbers(&[
            ("page", params.page),
            ("page_size", params.page_size),
        ]));
        let query: String = query_parts.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("&");
        let path = if query.is_empty() {
            "/automatcher/v1/receivables".to_string()
        } else {
            format!("/automatcher/v1/receivables?{}", query)
        };
        self.client.get(&path).await
    }

    /// Create whitelist entry for automatcher
    pub async fn create_whitelist(&self, request: &CreateWhitelistRequest) -> ApiResult<WhitelistResponse> {
        self.client.post("/automatcher/v1/whitelist", request).await
    }

    /// Get reconciliation rules
    pub async fn get_reconciliation_rules(&self) -> ApiResult<ReconciliationRulesResponse> {
        self.client.get("/automatcher/v1/reconciliation-rules").await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivablesParams {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivablesResponse {
    pub receivables: Vec<Receivable>,
    pub total_count: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receivable {
    pub id: String,
    pub amount: f64,
    pub account_number: String,
    pub bsb: String,
    pub received_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWhitelistRequest {
    pub account_number: String,
    pub bsb: String,
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistResponse {
    pub whitelist_id: String,
    pub account_number: String,
    pub bsb: String,
    pub account_name: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationRulesResponse {
    pub rules: Vec<ReconciliationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationRule {
    pub rule_id: String,
    pub name: String,
    pub pattern: String,
    pub active: bool,
}

