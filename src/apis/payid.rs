use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Pay ID API endpoints
pub struct PayIdApi<'a> {
    client: &'a Client,
}

impl<'a> PayIdApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Resolve Pay ID
    pub async fn resolve(&self, pay_id: &str) -> ApiResult<PayIdResolutionResponse> {
        self.client.get(&format!("/payid/v1/resolve/{}", pay_id)).await
    }

    /// Create Pay ID payment
    pub async fn create_payment(&self, request: &PayIdPaymentRequest) -> ApiResult<PayIdPaymentResponse> {
        self.client.post("/payid/v1/payment", request).await
    }

    /// Get Pay ID payment status
    pub async fn get_payment_status(&self, payment_id: &str) -> ApiResult<PayIdPaymentStatusResponse> {
        self.client
            .get(&format!("/payid/v1/payment/{}", payment_id))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdResolutionResponse {
    pub pay_id: String,
    pub account_number: String,
    pub bsb: String,
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdPaymentRequest {
    pub pay_id: String,
    pub amount: f64,
    pub description: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdPaymentResponse {
    pub payment_id: String,
    pub status: String,
    pub amount: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayIdPaymentStatusResponse {
    pub payment_id: String,
    pub status: String,
    pub amount: f64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

