use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Subscriptions API endpoints
pub struct SubscriptionsApi<'a> {
    client: &'a Client,
}

impl<'a> SubscriptionsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create webhook subscription
    pub async fn create(&self, request: &CreateWebhookSubscriptionRequest) -> ApiResult<WebhookSubscriptionResponse> {
        self.client.post("/subscriptions/v1/create", request).await
    }

    /// Update webhook subscription
    pub async fn update(&self, request: &UpdateWebhookSubscriptionRequest) -> ApiResult<WebhookSubscriptionResponse> {
        self.client.post("/subscriptions/v1/update", request).await
    }

    /// List webhook subscriptions
    pub async fn list(&self) -> ApiResult<WebhookListResponse> {
        self.client.get("/subscriptions/v1/list").await
    }

    /// Delete webhook subscription
    pub async fn delete(&self, id: u64) -> ApiResult<DeleteWebhookSubscriptionResponse> {
        self.client
            .delete(&format!("/subscriptions/v1/delete/{}", id))
            .await
    }

    /// Resend webhook
    pub async fn resend(&self, webhook_id: u64) -> ApiResult<ResendWebhookResponse> {
        self.client
            .post(&format!("/subscriptions/v2/resend/{}", webhook_id), &())
            .await
    }

    /// Get webhook report
    pub async fn get_report(&self, date: &str, skip: Option<u32>, take: Option<u32>) -> ApiResult<WebhookReportResponse> {
        let skip_val = skip.unwrap_or(1);
        let take_val = take.unwrap_or(100);
        self.client
            .get(&format!("/subscriptions/v1/report/{}?skip={}&take={}", date, skip_val, take_val))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookSubscriptionRequest {
    pub event_name: String,
    pub target_url: String,
    pub subscription_status: String, // "On" or "Off"
    pub security_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookSubscriptionRequest {
    pub id: u64,
    pub target_url: Option<String>,
    pub subscription_status: Option<String>,
    pub security_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscriptionResponse {
    pub id: Option<u64>,
    pub webhook_url: Option<String>,
    pub event_names: Option<String>,
    pub subscription_name: Option<String>,
    pub disabled: Option<String>,
    pub security_token: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookListResponse {
    pub items: Option<Vec<WebhookSubscriptionResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookSubscriptionResponse {
    pub status: String,
    pub status_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendWebhookResponse {
    pub status: String,
    pub status_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookReportResponse {
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

