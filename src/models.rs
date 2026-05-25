use serde::{Deserialize, Serialize};

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub status: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            message: None,
            status: None,
        }
    }
}

/// Common pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Common date range parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeParams {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

