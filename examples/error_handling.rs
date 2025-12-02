//! Example: Error handling
//!
//! Run with: cargo run --example error_handling

use server_r_client::{ApiClient, ApiError, Config, NodeType};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        std::env::var("API_HOST").unwrap_or_else(|_| "https://api.example.com".to_string()),
        std::env::var("API_TOKEN").unwrap_or_else(|_| "your-api-token".to_string()),
    )
    .with_timeout(Duration::from_secs(10));

    let client = ApiClient::new(config)?;

    let node_type = NodeType::Trojan;
    let register_id = "your-register-id";

    // Demonstrate different error types
    println!("--- Error handling examples ---\n");

    // Example 1: Handle users with ETag (304 Not Modified)
    println!("1. Handle NotModified error:");
    match client.users(node_type, register_id).await {
        Ok(users) => println!("   Got {} users", users.len()),
        Err(ApiError::NotModified { url }) => {
            println!("   Users not modified (cached)");
            println!("   URL: {}", url);
        }
        Err(e) => println!("   Other error: {}", e),
    }

    // Example 2: Match on specific error types
    println!("\n2. Match on error types:");
    match client.config(node_type, 1).await {
        Ok(config) => println!("   Got config: {}", config.type_name()),
        Err(ApiError::ServerError {
            status_code,
            message,
            url,
        }) => {
            println!("   Server error!");
            println!("   Status: {}", status_code);
            println!("   Message: {}", message);
            println!("   URL: {}", url);
        }
        Err(ApiError::NetworkError { message, url, .. }) => {
            println!("   Network error!");
            println!("   Message: {}", message);
            println!("   URL: {}", url);
        }
        Err(ApiError::ParseError { message, url, .. }) => {
            println!("   Parse error!");
            println!("   Message: {}", message);
            println!("   URL: {}", url);
        }
        Err(e) => println!("   Other error: {}", e),
    }

    // Example 3: Use error helper methods
    println!("\n3. Use error helper methods:");
    match client.heartbeat(node_type, register_id).await {
        Ok(()) => println!("   Heartbeat sent"),
        Err(ref e) if e.is_server_error() => {
            println!("   Server error occurred");
            println!("   Error type: {:?}", e.error_type());
        }
        Err(ref e) if e.is_network_error() => {
            println!("   Network error occurred");
            println!("   Error type: {:?}", e.error_type());
        }
        Err(ref e) if e.is_not_modified() => {
            println!("   Not modified");
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 4: Convert error to string
    println!("\n4. Error as string:");
    let err = ApiError::from_status_code(404, "Node not found", "http://api.example.com/node/1");
    println!("   {}", err);

    // Example 5: Create custom errors
    println!("\n5. Create custom errors:");
    let network_err = ApiError::network_error("Connection timeout", "http://api.example.com", None);
    println!("   Network: {}", network_err);

    let parse_err = ApiError::parse_error("Invalid JSON", "http://api.example.com", None);
    println!("   Parse: {}", parse_err);

    let config_err = ApiError::config_error("Invalid API host");
    println!("   Config: {}", config_err);

    Ok(())
}
