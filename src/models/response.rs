use serde::{Deserialize, Serialize};

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(default)]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: Some(message.into()),
        }
    }
}

/// Register response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponseData {
    pub register_id: String,
}

/// Verify response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponseData {
    pub valid: bool,
}

/// Empty response data (for operations that don't return data)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyData {}

/// Response type aliases
pub type RegisterResponse = ApiResponse<RegisterResponseData>;
pub type VerifyResponse = ApiResponse<VerifyResponseData>;
pub type EmptyResponse = ApiResponse<EmptyData>;

/// Users response with ETag information
#[derive(Debug, Clone)]
pub struct UsersResponse<T> {
    pub data: T,
    pub etag: Option<String>,
}

impl<T> UsersResponse<T> {
    pub fn new(data: T, etag: Option<String>) -> Self {
        Self { data, etag }
    }
}
