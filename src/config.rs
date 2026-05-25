
/// API environment configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Production environment
    Production,
    /// Sandbox environment
    Sandbox,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => "https://api.mpay.com.au",
            Environment::Sandbox => "https://api.m-pay.com.au",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Production
    }
}

/// Basic authentication credentials
#[derive(Debug, Clone)]
pub struct BasicAuthCredentials {
    pub username: String,
    pub password: String,
}

impl BasicAuthCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn from_api_key(api_key: String) -> Self {
        Self {
            username: api_key,
            password: String::new(),
        }
    }
}

/// HTTP client configuration options
#[derive(Debug, Clone)]
pub struct HttpClientOptions {
    pub timeout: Option<u64>,
    pub retry_config: Option<RetryConfiguration>,
}

impl Default for HttpClientOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            retry_config: None,
        }
    }
}

/// Retry configuration for failed requests
#[derive(Debug, Clone)]
pub struct RetryConfiguration {
    pub max_number_of_retries: u32,
    pub retry_on_timeout: bool,
    pub retry_interval: u64,
    pub maximum_retry_wait_time: u64,
    pub backoff_factor: u32,
    pub http_status_codes_to_retry: Vec<u16>,
    pub http_methods_to_retry: Vec<String>,
}

impl Default for RetryConfiguration {
    fn default() -> Self {
        Self {
            max_number_of_retries: 0,
            retry_on_timeout: true,
            retry_interval: 1,
            maximum_retry_wait_time: 0,
            backoff_factor: 2,
            http_status_codes_to_retry: vec![408, 413, 429, 500, 502, 503, 504, 521, 522, 524],
            http_methods_to_retry: vec!["GET".to_string(), "PUT".to_string()],
        }
    }
}

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub environment: Environment,
    pub basic_auth_credentials: BasicAuthCredentials,
    pub timeout: Option<u64>,
    pub http_client_options: Option<HttpClientOptions>,
}

impl ClientConfig {
    pub fn new(
        basic_auth_credentials: BasicAuthCredentials,
        environment: Option<Environment>,
        timeout: Option<u64>,
        http_client_options: Option<HttpClientOptions>,
    ) -> Self {
        Self {
            environment: environment.unwrap_or_default(),
            basic_auth_credentials,
            timeout,
            http_client_options,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        Self {
            environment: Environment::default(),
            basic_auth_credentials: BasicAuthCredentials::from_api_key(api_key),
            timeout: None,
            http_client_options: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_base_url() {
        assert_eq!(Environment::Production.base_url(), "https://api.mpay.com.au");
        assert_eq!(Environment::Sandbox.base_url(), "https://api.m-pay.com.au");
    }

    #[test]
    fn test_environment_default() {
        assert_eq!(Environment::default(), Environment::Production);
    }

    #[test]
    fn test_basic_auth_credentials_from_api_key() {
        let creds = BasicAuthCredentials::from_api_key("test-key".to_string());
        assert_eq!(creds.username, "test-key");
        assert_eq!(creds.password, "");
    }

    #[test]
    fn test_basic_auth_credentials_new() {
        let creds = BasicAuthCredentials::new("user".to_string(), "pass".to_string());
        assert_eq!(creds.username, "user");
        assert_eq!(creds.password, "pass");
    }

    #[test]
    fn test_retry_configuration_default() {
        let config = RetryConfiguration::default();
        assert_eq!(config.max_number_of_retries, 0);
        assert!(config.retry_on_timeout);
        assert_eq!(config.retry_interval, 1);
        assert_eq!(config.backoff_factor, 2);
    }

    #[test]
    fn test_client_config_with_api_key() {
        let config = ClientConfig::with_api_key("test-key".to_string());
        assert_eq!(config.basic_auth_credentials.username, "test-key");
        assert_eq!(config.environment, Environment::Production);
    }

    #[test]
    fn test_client_config_new() {
        let creds = BasicAuthCredentials::from_api_key("test-key".to_string());
        let config = ClientConfig::new(creds, Some(Environment::Sandbox), Some(60), None);
        assert_eq!(config.environment, Environment::Sandbox);
        assert_eq!(config.timeout, Some(60));
    }
}
