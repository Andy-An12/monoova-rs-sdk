use crate::apis::{
    AutomatcherApi, BpayApi, FinancialApi, MAccountApi, MWalletApi, PayIdApi, ReceivablesApi,
    ReportsApi, SecurityApi, SubscriptionsApi, TokenApi, ToolsApi, VerifyApi, WebhooksApi,
};
use crate::config::{ClientConfig, Environment};
use crate::error::{ApiError, ApiResult};
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response};
use std::time::Duration;
use url::Url;

/// Main API client for Monoova Payments API
pub struct Client {
    config: ClientConfig,
    http_client: ReqwestClient,
    base_url: String,
}

impl Client {
    /// Create a new client with the given configuration
    pub fn new(config: ClientConfig) -> ApiResult<Self> {
        let base_url = config.environment.base_url().to_string();
        
        let client_builder = ReqwestClient::builder()
            .timeout(Duration::from_secs(
                config.timeout.unwrap_or(30),
            ))
            .danger_accept_invalid_certs(false);

        let http_client = client_builder.build()?;

        Ok(Self {
            config,
            http_client,
            base_url,
        })
    }

    /// Create a new client with API key (simplified constructor)
    pub fn with_api_key(api_key: String) -> ApiResult<Self> {
        Self::new(ClientConfig::with_api_key(api_key))
    }

    /// Create a new client with API key and environment
    pub fn with_api_key_and_env(api_key: String, environment: Environment) -> ApiResult<Self> {
        let mut config = ClientConfig::with_api_key(api_key);
        config.environment = environment;
        Self::new(config)
    }

    /// Create a new client with custom base URL (for testing)
    pub fn with_base_url(api_key: String, base_url: String) -> ApiResult<Self> {
        let config = ClientConfig::with_api_key(api_key);
        let mut client = Self::new(config)?;
        client.base_url = base_url;
        Ok(client)
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the current environment
    pub fn environment(&self) -> Environment {
        self.config.environment
    }

    /// Get Financial API
    pub fn financial(&self) -> FinancialApi<'_> {
        FinancialApi::new(self)
    }

    /// Get BPAY API
    pub fn bpay(&self) -> BpayApi<'_> {
        BpayApi::new(self)
    }

    /// Get Verify API
    pub fn verify(&self) -> VerifyApi<'_> {
        VerifyApi::new(self)
    }

    /// Get Pay ID API
    pub fn payid(&self) -> PayIdApi<'_> {
        PayIdApi::new(self)
    }

    /// Get Tools API
    pub fn tools(&self) -> ToolsApi<'_> {
        ToolsApi::new(self)
    }

    /// Get Subscriptions API
    pub fn subscriptions(&self) -> SubscriptionsApi<'_> {
        SubscriptionsApi::new(self)
    }

    /// Get Webhooks API
    pub fn webhooks(&self) -> WebhooksApi<'_> {
        WebhooksApi::new(self)
    }

    /// Get Reports API
    pub fn reports(&self) -> ReportsApi<'_> {
        ReportsApi::new(self)
    }

    /// Get Token API
    pub fn token(&self) -> TokenApi<'_> {
        TokenApi::new(self)
    }

    /// Get Security API
    pub fn security(&self) -> SecurityApi<'_> {
        SecurityApi::new(self)
    }

    /// Get M Account API
    pub fn m_account(&self) -> MAccountApi<'_> {
        MAccountApi::new(self)
    }

    /// Get M Wallet API
    pub fn m_wallet(&self) -> MWalletApi<'_> {
        MWalletApi::new(self)
    }

    /// Get Automatcher API
    pub fn automatcher(&self) -> AutomatcherApi<'_> {
        AutomatcherApi::new(self)
    }

    /// Get Receivables API
    pub fn receivables(&self) -> ReceivablesApi<'_> {
        ReceivablesApi::new(self)
    }

    /// Build a request with authentication
    fn build_request(&self, method: Method, path: &str) -> ApiResult<RequestBuilder> {
        let url = Url::parse(&self.base_url)?
            .join(path)
            .map_err(|e| ApiError::InvalidUrl(url::ParseError::from(e)))?;

        let mut request = self.http_client.request(method, url);

        // Add basic authentication
        request = request.basic_auth(
            &self.config.basic_auth_credentials.username,
            Some(&self.config.basic_auth_credentials.password),
        );

        Ok(request)
    }
    
    /// Get the full URL for a path (for debugging)
    pub fn get_full_url(&self, path: &str) -> ApiResult<String> {
        let url = Url::parse(&self.base_url)?
            .join(path)
            .map_err(|e| ApiError::InvalidUrl(url::ParseError::from(e)))?;
        Ok(url.to_string())
    }

    /// Execute a GET request
    pub async fn get<T>(&self, path: &str) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.execute_request(Method::GET, path, None::<&()>).await
    }

    /// Execute a POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        B: serde::Serialize,
    {
        self.execute_request(Method::POST, path, Some(body)).await
    }

    /// Execute a PUT request
    pub async fn put<T, B>(&self, path: &str, body: &B) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        B: serde::Serialize,
    {
        self.execute_request(Method::PUT, path, Some(body)).await
    }

    /// Execute a DELETE request
    pub async fn delete<T>(&self, path: &str) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.execute_request(Method::DELETE, path, None::<&()>).await
    }

    /// Execute a request with optional body
    async fn execute_request<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        B: serde::Serialize,
    {
        let mut request = self.build_request(method.clone(), path)?;

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = self.send_request(request).await?;
        self.parse_response(response).await
    }

    /// Send the request with retry logic
    async fn send_request(&self, request: RequestBuilder) -> ApiResult<Response> {
        let retry_config = self
            .config
            .http_client_options
            .as_ref()
            .and_then(|opts| opts.retry_config.as_ref());

        let mut last_error = None;
        let max_retries = retry_config
            .map(|c| c.max_number_of_retries)
            .unwrap_or(0);

        for attempt in 0..=max_retries {
            match request.try_clone() {
                Some(req) => {
                    match req.send().await {
                        Ok(response) => {
                            let status = response.status();
                            
                            // Check if we should retry based on status code
                            if attempt < max_retries {
                                if let Some(retry_config) = retry_config {
                                    if retry_config.http_status_codes_to_retry.contains(&status.as_u16()) {
                                        // Wait before retrying
                                        let wait_time = self.calculate_wait_time(attempt, retry_config);
                                        tokio::time::sleep(Duration::from_secs(wait_time)).await;
                                        continue;
                                    }
                                }
                            }

                            return Ok(response);
                        }
                        Err(e) => {
                            let should_retry = if attempt < max_retries {
                                if let Some(retry_config) = retry_config {
                                    retry_config.retry_on_timeout || !e.is_timeout()
                                } else {
                                    false
                                }
                            } else {
                                false
                            };
                            
                            if should_retry {
                                if let Some(retry_config) = retry_config {
                                    let wait_time = self.calculate_wait_time(attempt, retry_config);
                                    tokio::time::sleep(Duration::from_secs(wait_time)).await;
                                    last_error = Some(e);
                                    continue;
                                }
                            }
                            last_error = Some(e);
                        }
                    }
                }
                None => {
                    // Request cannot be cloned, execute once
                    return request.send().await.map_err(ApiError::Http);
                }
            }
        }

        Err(last_error.map(ApiError::Http).unwrap_or_else(|| {
            ApiError::Unknown("Request failed after retries".to_string())
        }))
    }

    /// Calculate wait time for retry with exponential backoff
    fn calculate_wait_time(&self, attempt: u32, retry_config: &crate::config::RetryConfiguration) -> u64 {
        let base_wait = retry_config.retry_interval;
        let backoff = retry_config.backoff_factor.pow(attempt);
        let wait_time = base_wait * backoff as u64;
        
        if retry_config.maximum_retry_wait_time > 0 {
            wait_time.min(retry_config.maximum_retry_wait_time)
        } else {
            wait_time
        }
    }

    /// Parse the response
    async fn parse_response<T>(&self, response: Response) -> ApiResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            if text.is_empty() {
                return Err(ApiError::Unknown("Empty response body".to_string()));
            }
            serde_json::from_str(&text).map_err(ApiError::Serialization)
        } else {
            Err(ApiError::from_response(
                status.as_u16(),
                text,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::with_api_key("test-api-key".to_string());
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_with_env() {
        let client = Client::with_api_key_and_env(
            "test-api-key".to_string(),
            Environment::Sandbox,
        );
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.environment(), Environment::Sandbox);
        assert_eq!(client.base_url(), "https://api.m-pay.com.au");
    }

    #[test]
    fn test_environment_base_url() {
        assert_eq!(Environment::Production.base_url(), "https://api.mpay.com.au");
        assert_eq!(Environment::Sandbox.base_url(), "https://api.m-pay.com.au");
    }
}

