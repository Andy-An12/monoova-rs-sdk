use crate::apis::utils;
use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Financial API endpoints
pub struct FinancialApi<'a> {
    client: &'a Client,
}

impl<'a> FinancialApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get account balance
    pub async fn get_balance(&self) -> ApiResult<BalanceResponse> {
        self.client.get("/financial/v1/balance").await
    }

    /// Get account statement
    pub async fn get_statement(&self, params: &StatementParams) -> ApiResult<StatementResponse> {
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
            "/financial/v1/statement".to_string()
        } else {
            format!("/financial/v1/statement?{}", query)
        };
        self.client.get(&path).await
    }

    /// Get transaction details
    pub async fn get_transaction(&self, transaction_id: &str) -> ApiResult<TransactionResponse> {
        self.client
            .get(&format!("/financial/v1/transaction/{}", transaction_id))
            .await
    }

    /// Get transactions list
    pub async fn get_transactions(&self, params: &TransactionParams) -> ApiResult<TransactionsResponse> {
        let mut query_parts = Vec::new();
        query_parts.push(utils::build_query_string(&[
            ("from_date", params.from_date.clone()),
            ("to_date", params.to_date.clone()),
            ("transaction_type", params.transaction_type.clone()),
        ]));
        query_parts.push(utils::build_query_string_with_numbers(&[
            ("page", params.page),
            ("page_size", params.page_size),
        ]));
        let query: String = query_parts.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("&");
        let path = if query.is_empty() {
            "/financial/v1/transactions".to_string()
        } else {
            format!("/financial/v1/transactions?{}", query)
        };
        self.client.get(&path).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
    pub available_balance: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementParams {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementResponse {
    pub transactions: Vec<Transaction>,
    pub total_count: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionParams {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub transaction_type: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub total_count: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transaction: Transaction,
}

