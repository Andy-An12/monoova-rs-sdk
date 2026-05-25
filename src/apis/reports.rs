use crate::client::Client;
use crate::error::ApiResult;
use serde::{Deserialize, Serialize};

/// Reports API endpoints
pub struct ReportsApi<'a> {
    client: &'a Client,
}

impl<'a> ReportsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Generate a report
    pub async fn generate(&self, request: &GenerateReportRequest) -> ApiResult<ReportResponse> {
        self.client.post("/reports/v1/generate", request).await
    }

    /// Get report status
    pub async fn get_status(&self, report_id: &str) -> ApiResult<ReportStatusResponse> {
        self.client
            .get(&format!("/reports/v1/{}", report_id))
            .await
    }

    /// Download report
    pub async fn download(&self, report_id: &str) -> ApiResult<Vec<u8>> {
        let _response = self.client
            .get::<serde_json::Value>(&format!("/reports/v1/{}/download", report_id))
            .await?;
        // In a real implementation, this would return binary data
        Ok(vec![])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateReportRequest {
    pub report_type: String,
    pub from_date: String,
    pub to_date: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    pub report_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatusResponse {
    pub report_id: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub download_url: Option<String>,
}

