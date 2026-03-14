use reqwest::StatusCode;
use thiserror::Error;

/// Unified error type for all API operations
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("HTTP error: {status} - {message}")]
    Http {
        status: StatusCode,
        message: String,
    },

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimit { retry_after: u64 },

    #[error("Authentication required")]
    Unauthorized,

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Server error: {status} - {message}")]
    ServerError {
        status: StatusCode,
        message: String,
    },

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Timeout exceeded")]
    Timeout,

    #[error("Retry limit exceeded after {attempts} attempts")]
    RetryLimitExceeded { attempts: u32 },

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("API error: {code} - {message}")]
    Api {
        code: String,
        message: String,
    },
}

impl ApiError {
    /// Create HTTP error from status code and message
    pub fn http(status: StatusCode, message: impl Into<String>) -> Self {
        ApiError::Http {
            status,
            message: message.into(),
        }
    }

    /// Create server error from status code and message
    pub fn server_error(status: StatusCode, message: impl Into<String>) -> Self {
        ApiError::ServerError {
            status,
            message: message.into(),
        }
    }

    /// Create API error from code and message
    pub fn api(code: impl Into<String>, message: impl Into<String>) -> Self {
        ApiError::Api {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Check if error is retryable (429, 502, 503, 504)
    pub fn is_retryable(&self) -> bool {
        match self {
            ApiError::Http { status, .. } | ApiError::ServerError { status, .. } => {
                matches!(status.as_u16(), 429 | 502 | 503 | 504)
            }
            ApiError::Network(e) => e.is_timeout() || e.is_connect(),
            ApiError::Timeout => true,
            _ => false,
        }
    }

    /// Get retry-after duration if available
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            ApiError::RateLimit { retry_after } => Some(*retry_after),
            ApiError::Http { status, .. } | ApiError::ServerError { status, .. } => {
                if status.as_u16() == 429 {
                    Some(1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;
