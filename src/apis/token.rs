use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Token API endpoints
pub struct TokenApi<'a> {
    client: &'a Client,
}

impl<'a> TokenApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a token
    pub async fn create(&self, request: &CreateTokenRequest) -> ApiResult<TokenResponse> {
        self.client.post("/token/v1", request).await
    }

    /// Get token details
    pub async fn get(&self, token_id: &str) -> ApiResult<TokenResponse> {
        self.client.get(&format!("/token/v1/{}", token_id)).await
    }

    /// Revoke a token
    pub async fn revoke(&self, token_id: &str) -> ApiResult<RevokeTokenResponse> {
        self.client
            .delete(&format!("/token/v1/{}", token_id))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub expires_in: Option<u64>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token_id: String,
    pub token: String,
    pub name: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTokenResponse {
    pub token_id: String,
    pub revoked: bool,
    pub revoked_at: String,
}

