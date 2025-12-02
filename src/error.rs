use thiserror::Error;

/// Error types for API operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    /// HTTP 4xx/5xx errors
    ServerError,
    /// Connection/network failures
    NetworkError,
    /// JSON parsing failures
    ParseError,
    /// HTTP 304 Not Modified
    NotModified,
    /// Unexpected errors
    Unknown,
}

/// API error with detailed information
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Server error (status {status_code}): {message} - URL: {url}")]
    ServerError {
        status_code: u16,
        message: String,
        url: String,
    },

    #[error("Network error: {message} - URL: {url}")]
    NetworkError {
        message: String,
        url: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("Parse error: {message} - URL: {url}")]
    ParseError {
        message: String,
        url: String,
        #[source]
        source: Option<serde_json::Error>,
    },

    #[error("Not modified (304) - URL: {url}")]
    NotModified { url: String },

    #[error("Unknown error: {message}")]
    Unknown { message: String },

    #[error("Invalid configuration: {message}")]
    ConfigError { message: String },

    #[error("Type conversion error: expected {expected}, got {actual}")]
    TypeConversionError { expected: String, actual: String },
}

impl ApiError {
    /// Get the error type
    pub fn error_type(&self) -> ErrorType {
        match self {
            ApiError::ServerError { .. } => ErrorType::ServerError,
            ApiError::NetworkError { .. } => ErrorType::NetworkError,
            ApiError::ParseError { .. } => ErrorType::ParseError,
            ApiError::NotModified { .. } => ErrorType::NotModified,
            ApiError::Unknown { .. }
            | ApiError::ConfigError { .. }
            | ApiError::TypeConversionError { .. } => ErrorType::Unknown,
        }
    }

    /// Check if this is a server error (4xx/5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self, ApiError::ServerError { .. })
    }

    /// Check if this is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, ApiError::NetworkError { .. })
    }

    /// Check if this is a parse error
    pub fn is_parse_error(&self) -> bool {
        matches!(self, ApiError::ParseError { .. })
    }

    /// Check if this is a 304 Not Modified response
    pub fn is_not_modified(&self) -> bool {
        matches!(self, ApiError::NotModified { .. })
    }

    /// Create a server error from status code
    pub fn from_status_code(
        status_code: u16,
        message: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        ApiError::ServerError {
            status_code,
            message: message.into(),
            url: url.into(),
        }
    }

    /// Create a network error
    pub fn network_error(
        message: impl Into<String>,
        url: impl Into<String>,
        source: Option<reqwest::Error>,
    ) -> Self {
        ApiError::NetworkError {
            message: message.into(),
            url: url.into(),
            source,
        }
    }

    /// Create a parse error
    pub fn parse_error(
        message: impl Into<String>,
        url: impl Into<String>,
        source: Option<serde_json::Error>,
    ) -> Self {
        ApiError::ParseError {
            message: message.into(),
            url: url.into(),
            source,
        }
    }

    /// Create a not modified error
    pub fn not_modified(url: impl Into<String>) -> Self {
        ApiError::NotModified { url: url.into() }
    }

    /// Create a config error
    pub fn config_error(message: impl Into<String>) -> Self {
        ApiError::ConfigError {
            message: message.into(),
        }
    }

    /// Create a type conversion error
    pub fn type_conversion_error(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        ApiError::TypeConversionError {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}

/// Result type alias for API operations
pub type Result<T> = std::result::Result<T, ApiError>;
