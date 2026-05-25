use crate::apis::utils;
use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// M Wallet API endpoints
pub struct MWalletApi<'a> {
    client: &'a Client,
}

impl<'a> MWalletApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create an M Wallet
    pub async fn create(&self, request: &CreateMWalletRequest) -> ApiResult<MWalletResponse> {
        self.client.post("/mwallet/v1", request).await
    }

    /// Get M Wallet details
    pub async fn get(&self, wallet_id: &str) -> ApiResult<MWalletResponse> {
        self.client
            .get(&format!("/mwallet/v1/{}", wallet_id))
            .await
    }

    /// List M Wallets
    pub async fn list(&self, params: &MWalletListParams) -> ApiResult<MWalletListResponse> {
        let query = utils::build_query_string_with_numbers(&[
            ("page", params.page),
            ("page_size", params.page_size),
        ]);
        let path = if query.is_empty() {
            "/mwallet/v1".to_string()
        } else {
            format!("/mwallet/v1?{}", query)
        };
        self.client.get(&path).await
    }

    /// Transfer funds between wallets
    pub async fn transfer(&self, request: &WalletTransferRequest) -> ApiResult<WalletTransferResponse> {
        self.client.post("/mwallet/v1/transfer", request).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMWalletRequest {
    pub wallet_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MWalletResponse {
    pub wallet_id: String,
    pub wallet_name: String,
    pub balance: f64,
    pub currency: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MWalletListParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MWalletListResponse {
    pub wallets: Vec<MWalletResponse>,
    pub total_count: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransferRequest {
    pub from_wallet_id: String,
    pub to_wallet_id: String,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransferResponse {
    pub transfer_id: String,
    pub from_wallet_id: String,
    pub to_wallet_id: String,
    pub amount: f64,
    pub status: String,
    pub created_at: String,
}

