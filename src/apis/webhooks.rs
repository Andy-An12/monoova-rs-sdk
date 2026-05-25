use crate::apis::utils;
use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Webhooks API endpoints
pub struct WebhooksApi<'a> {
    client: &'a Client,
}

impl<'a> WebhooksApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a webhook
    pub async fn create(&self, request: &CreateWebhookRequest) -> ApiResult<WebhookResponse> {
        self.client.post("/webhooks/v1", request).await
    }

    /// Get webhook details
    pub async fn get(&self, webhook_id: &str) -> ApiResult<WebhookResponse> {
        self.client.get(&format!("/webhooks/v1/{}", webhook_id)).await
    }

    /// List webhooks
    pub async fn list(&self, params: &WebhookListParams) -> ApiResult<WebhookListResponse> {
        let active_str = params.active.map(|a| if a { "true".to_string() } else { "false".to_string() });
        let mut query_parts = Vec::new();
        query_parts.push(utils::build_query_string(&[
            ("active", active_str),
        ]));
        query_parts.push(utils::build_query_string_with_numbers(&[
            ("page", params.page),
            ("page_size", params.page_size),
        ]));
        let query: String = query_parts.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("&");
        let path = if query.is_empty() {
            "/webhooks/v1".to_string()
        } else {
            format!("/webhooks/v1?{}", query)
        };
        self.client.get(&path).await
    }

    /// Update a webhook
    pub async fn update(&self, webhook_id: &str, request: &UpdateWebhookRequest) -> ApiResult<WebhookResponse> {
        self.client
            .put(&format!("/webhooks/v1/{}", webhook_id), request)
            .await
    }

    /// Delete a webhook
    pub async fn delete(&self, webhook_id: &str) -> ApiResult<DeleteWebhookResponse> {
        self.client
            .delete(&format!("/webhooks/v1/{}", webhook_id))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub webhook_id: String,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookListParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookListResponse {
    pub webhooks: Vec<WebhookResponse>,
    pub total_count: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookResponse {
    pub webhook_id: String,
    pub deleted: bool,
}

