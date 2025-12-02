//! # server-r-client
//!
//! Rust HTTP client library for xflash-panda server communication.
//!
//! This library provides a type-safe API client for communicating with a centralized
//! proxy server management panel. It supports various proxy protocols including
//! Trojan, ShadowSocks, Hysteria, Hysteria2, VMess, and AnyTLS.
//!
//! ## Features
//!
//! - Node lifecycle management (registration, verification, unregistration)
//! - Configuration retrieval for various proxy protocols
//! - User list management with ETag caching
//! - Traffic statistics reporting
//! - Heartbeat/health check functionality
//!
//! ## Example
//!
//! ```rust,no_run
//! use server_r_client::{ApiClient, Config, NodeType, RegisterRequest};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client configuration
//!     let config = Config::new("https://api.example.com", "your-api-token")
//!         .with_timeout(Duration::from_secs(10))
//!         .with_debug(true);
//!
//!     // Create the API client
//!     let client = ApiClient::new(config)?;
//!
//!     // Register a node
//!     let register_id = client
//!         .register(
//!             NodeType::Trojan,
//!             1,
//!             RegisterRequest::new("node.example.com", 443),
//!         )
//!         .await?;
//!
//!     println!("Registered with ID: {}", register_id);
//!
//!     // Get users
//!     let users = client.users(NodeType::Trojan, &register_id).await?;
//!     println!("Got {} users", users.len());
//!
//!     // Send heartbeat
//!     client.heartbeat(NodeType::Trojan, &register_id).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! The library provides detailed error types through the [`ApiError`] enum:
//!
//! - `ServerError` - HTTP 4xx/5xx errors
//! - `NetworkError` - Connection/network failures
//! - `ParseError` - JSON parsing failures
//! - `NotModified` - HTTP 304 (useful for ETag caching)
//!
//! ```rust,no_run
//! use server_r_client::{ApiClient, Config, NodeType, ApiError};
//!
//! async fn example(client: &ApiClient, register_id: &str) {
//!     match client.users(NodeType::Trojan, register_id).await {
//!         Ok(users) => println!("Got {} users", users.len()),
//!         Err(ApiError::NotModified { .. }) => println!("Users not modified"),
//!         Err(ApiError::ServerError { status_code, message, .. }) => {
//!             println!("Server error {}: {}", status_code, message);
//!         }
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```

mod client;
mod error;
pub mod models;

pub use client::{ApiClient, Config};
pub use error::{ApiError, ErrorType, Result};
pub use models::*;
