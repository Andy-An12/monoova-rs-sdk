use thiserror::Error;

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// API error types
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error: status={status}, message={message}")]
    Api { status: u16, message: String },

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Request timeout")]
    Timeout,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ApiError {
    pub fn from_response(status: u16, message: String) -> Self {
        Self::Api { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_from_response() {
        let error = ApiError::from_response(404, "Not Found".to_string());
        match error {
            ApiError::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            _ => panic!("Expected ApiError::Api"),
        }
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::Timeout;
        assert_eq!(error.to_string(), "Request timeout");

        let error = ApiError::Authentication("Invalid credentials".to_string());
        assert_eq!(error.to_string(), "Authentication error: Invalid credentials");

        let error = ApiError::Unknown("Something went wrong".to_string());
        assert_eq!(error.to_string(), "Unknown error: Something went wrong");
    }
}
