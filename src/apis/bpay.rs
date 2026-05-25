use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// BPAY API endpoints
pub struct BpayApi<'a> {
    client: &'a Client,
}

impl<'a> BpayApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a BPAY payment
    pub async fn create_payment(&self, request: &BpayPaymentRequest) -> ApiResult<BpayPaymentResponse> {
        self.client.post("/bpay/v1/payment", request).await
    }

    /// Get BPAY payment status
    pub async fn get_payment_status(&self, payment_id: &str) -> ApiResult<BpayPaymentStatusResponse> {
        self.client
            .get(&format!("/bpay/v1/payment/{}", payment_id))
            .await
    }

    /// Get BPAY biller details
    pub async fn get_biller_details(&self, biller_code: &str) -> ApiResult<BillerDetailsResponse> {
        self.client
            .get(&format!("/bpay/v1/biller/{}", biller_code))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpayPaymentRequest {
    pub amount: f64,
    pub biller_code: String,
    pub customer_reference: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpayPaymentResponse {
    pub payment_id: String,
    pub status: String,
    pub amount: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpayPaymentStatusResponse {
    pub payment_id: String,
    pub status: String,
    pub amount: f64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillerDetailsResponse {
    pub biller_code: String,
    pub biller_name: String,
    pub active: bool,
}

