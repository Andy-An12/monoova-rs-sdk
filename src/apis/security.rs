use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Security API endpoints
pub struct SecurityApi<'a> {
    client: &'a Client,
}

impl<'a> SecurityApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Change password
    pub async fn change_password(&self, request: &ChangePasswordRequest) -> ApiResult<ChangePasswordResponse> {
        self.client.post("/security/v1/password", request).await
    }

    /// Generate OneShot security token
    pub async fn generate_oneshot_token(&self) -> ApiResult<OneShotTokenResponse> {
        self.client.post("/security/v1/oneshot", &()).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneShotTokenResponse {
    pub token: String,
    pub expires_at: String,
}

