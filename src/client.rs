use reqwest::{Client as HttpClient, Response, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::error::{ApiError, Result};
use crate::models::*;

/// Client configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL of the API server
    pub api_host: String,
    /// API authentication token
    pub token: String,
    /// Request timeout (default: 5 seconds)
    pub timeout: Duration,
    /// Enable debug logging
    pub debug: bool,
}

impl Config {
    /// Create a new configuration
    pub fn new(api_host: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            api_host: api_host.into(),
            token: token.into(),
            timeout: Duration::from_secs(5),
            debug: false,
        }
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// API Client for xflash-panda server
#[derive(Clone)]
pub struct ApiClient {
    config: Config,
    http_client: HttpClient,
    etag_cache: Arc<RwLock<HashMap<String, String>>>,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(config: Config) -> Result<Self> {
        let http_client = HttpClient::builder()
            .timeout(config.timeout)
            .no_proxy()
            .build()
            .map_err(|e| ApiError::config_error(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            http_client,
            etag_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Build URL with query parameters
    fn build_url(&self, path: &str, params: &[(&str, &str)]) -> String {
        let mut url = format!("{}{}", self.config.api_host.trim_end_matches('/'), path);

        let mut query_params: Vec<(&str, &str)> = vec![("token", &self.config.token)];
        query_params.extend(params);

        if !query_params.is_empty() {
            url.push('?');
            let params_str: Vec<String> = query_params
                .iter()
                .map(|(k, v)| {
                    let encoded: String =
                        url::form_urlencoded::byte_serialize(v.as_bytes()).collect();
                    format!("{}={}", k, encoded)
                })
                .collect();
            url.push_str(&params_str.join("&"));
        }

        url
    }

    /// Make a GET request
    async fn get(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = self.build_url(path, params);

        if self.config.debug {
            debug!("GET {}", url);
        }

        let response = self
            .http_client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &url, Some(e)))?;

        self.check_response(response, &url).await
    }

    /// Make a GET request with ETag support
    async fn get_with_etag(
        &self,
        path: &str,
        params: &[(&str, &str)],
        cache_key: &str,
    ) -> Result<Response> {
        let url = self.build_url(path, params);

        if self.config.debug {
            debug!("GET (with ETag) {}", url);
        }

        let etag = self.etag_cache.read().await.get(cache_key).cloned();

        let mut request = self
            .http_client
            .get(&url)
            .header("Content-Type", "application/json");

        if let Some(etag) = &etag {
            request = request.header("If-None-Match", etag);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &url, Some(e)))?;

        if response.status() == StatusCode::NOT_MODIFIED {
            return Err(ApiError::not_modified(&url));
        }

        // Store the new ETag if present
        if let Some(new_etag) = response.headers().get("ETag") {
            if let Ok(etag_str) = new_etag.to_str() {
                self.etag_cache
                    .write()
                    .await
                    .insert(cache_key.to_string(), etag_str.to_string());
            }
        }

        self.check_response(response, &url).await
    }

    /// Make a POST request with JSON body
    async fn post<T: serde::Serialize>(
        &self,
        path: &str,
        params: &[(&str, &str)],
        body: &T,
    ) -> Result<Response> {
        let url = self.build_url(path, params);

        if self.config.debug {
            debug!("POST {}", url);
        }

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &url, Some(e)))?;

        self.check_response(response, &url).await
    }

    /// Check response status and handle errors
    async fn check_response(&self, response: Response, url: &str) -> Result<Response> {
        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else if status == StatusCode::NOT_MODIFIED {
            Err(ApiError::not_modified(url))
        } else {
            let status_code = status.as_u16();
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            error!("API error: {} - {} - {}", status_code, message, url);
            Err(ApiError::from_status_code(status_code, message, url))
        }
    }

    // ==================== Configuration APIs ====================

    /// Get raw node configuration
    pub async fn raw_config(&self, node_type: NodeType, node_id: i64) -> Result<Vec<u8>> {
        let path = format!("/api/v1/server/{}/config", node_type);
        let node_id_str = node_id.to_string();
        let params = [("node_id", node_id_str.as_str())];

        let response = self.get(&path, &params).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;

        Ok(bytes.to_vec())
    }

    /// Get parsed node configuration (enhanced)
    pub async fn config(&self, node_type: NodeType, node_id: i64) -> Result<NodeConfigEnum> {
        let path = format!("/api/v1/server/enhanced/{}/config", node_type);
        let node_id_str = node_id.to_string();
        let params = [("node_id", node_id_str.as_str())];

        let response = self.get(&path, &params).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;

        // Parse the response wrapper first
        let api_response: ApiResponse<serde_json::Value> = serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        // Then parse the config data
        let config_bytes = serde_json::to_vec(&api_response.data)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        parse_config(node_type, &config_bytes)
    }

    // ==================== Node Management APIs ====================

    /// Register a node with the server
    pub async fn register(
        &self,
        node_type: NodeType,
        node_id: i64,
        request: RegisterRequest,
    ) -> Result<String> {
        let path = format!("/api/v1/server/enhanced/{}/register", node_type);
        let node_id_str = node_id.to_string();
        let params = [("node_id", node_id_str.as_str())];

        let response = self.post(&path, &params, &request).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;
        let api_response: RegisterResponse = serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        Ok(api_response.data.register_id)
    }

    /// Unregister a node
    pub async fn unregister(&self, node_type: NodeType, register_id: &str) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/unregister", node_type);
        let params = [("register_id", register_id)];

        // Empty body for unregister
        let empty: HashMap<String, String> = HashMap::new();
        self.post(&path, &params, &empty).await?;

        Ok(())
    }

    /// Verify if a register_id is valid
    pub async fn verify(&self, node_type: NodeType, register_id: &str) -> Result<bool> {
        let path = format!("/api/v1/server/enhanced/{}/verify", node_type);
        let request = VerifyRequest::new(register_id);

        let response = self.post(&path, &[], &request).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;
        let api_response: VerifyResponse = serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        Ok(api_response.data.valid)
    }

    // ==================== User Management APIs ====================

    /// Get raw users data with ETag caching support
    pub async fn raw_users(&self, node_type: NodeType, register_id: &str) -> Result<Vec<u8>> {
        let path = format!("/api/v1/server/enhanced/{}/users", node_type);
        let params = [("register_id", register_id)];
        let cache_key = format!("{}:{}", node_type, register_id);

        let response = self.get_with_etag(&path, &params, &cache_key).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;

        Ok(bytes.to_vec())
    }

    /// Get parsed user list
    pub async fn users(&self, node_type: NodeType, register_id: &str) -> Result<Vec<User>> {
        let path = format!("/api/v1/server/enhanced/{}/users", node_type);
        let params = [("register_id", register_id)];
        let cache_key = format!("{}:{}", node_type, register_id);

        let response = self.get_with_etag(&path, &params, &cache_key).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;
        let api_response: ApiResponse<Vec<User>> = serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        Ok(api_response.data)
    }

    /// Get users with ETag information
    pub async fn users_with_etag(
        &self,
        node_type: NodeType,
        register_id: &str,
    ) -> Result<UsersResponse<Vec<User>>> {
        let path = format!("/api/v1/server/enhanced/{}/users", node_type);
        let params = [("register_id", register_id)];
        let cache_key = format!("{}:{}", node_type, register_id);

        let response = self.get_with_etag(&path, &params, &cache_key).await?;

        let etag = response
            .headers()
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::network_error(e.to_string(), &path, Some(e)))?;
        let api_response: ApiResponse<Vec<User>> = serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::parse_error(e.to_string(), &path, Some(e)))?;

        Ok(UsersResponse::new(api_response.data, etag))
    }

    // ==================== Traffic/Statistics APIs ====================

    /// Submit user traffic data
    pub async fn submit(
        &self,
        node_type: NodeType,
        register_id: &str,
        data: Vec<UserTraffic>,
    ) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/submit", node_type);
        let request = SubmitRequest::new(register_id, data);

        self.post(&path, &[], &request).await?;
        Ok(())
    }

    /// Submit traffic data with agent information
    pub async fn submit_with_agent(
        &self,
        node_type: NodeType,
        register_id: &str,
        data: Vec<UserTraffic>,
    ) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/submitWithAgent", node_type);
        let request = SubmitRequest::new(register_id, data);

        self.post(&path, &[], &request).await?;
        Ok(())
    }

    /// Submit aggregated traffic statistics
    pub async fn submit_stats_with_agent(
        &self,
        node_type: NodeType,
        register_id: &str,
        data: TrafficStats,
    ) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/submitStatsWithAgent", node_type);
        let request = SubmitStatsRequest::new(register_id, data);

        self.post(&path, &[], &request).await?;
        Ok(())
    }

    // ==================== Health Monitoring APIs ====================

    /// Send heartbeat to server
    pub async fn heartbeat(&self, node_type: NodeType, register_id: &str) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/heartbeat", node_type);
        let request = HeartbeatRequest::new(register_id);

        self.post(&path, &[], &request).await?;
        Ok(())
    }

    /// Send heartbeat with node IP
    pub async fn heartbeat_with_ip(
        &self,
        node_type: NodeType,
        register_id: &str,
        node_ip: &str,
    ) -> Result<()> {
        let path = format!("/api/v1/server/enhanced/{}/heartbeat", node_type);
        let request = HeartbeatRequest::new(register_id).with_node_ip(node_ip);

        self.post(&path, &[], &request).await?;
        Ok(())
    }

    // ==================== Utility Methods ====================

    /// Clear the ETag cache
    pub async fn clear_etag_cache(&self) {
        self.etag_cache.write().await.clear();
    }

    /// Get the current ETag for a cache key
    pub async fn get_etag(&self, node_type: NodeType, register_id: &str) -> Option<String> {
        let cache_key = format!("{}:{}", node_type, register_id);
        self.etag_cache.read().await.get(&cache_key).cloned()
    }
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("api_host", &self.config.api_host)
            .field("timeout", &self.config.timeout)
            .field("debug", &self.config.debug)
            .finish()
    }
}
