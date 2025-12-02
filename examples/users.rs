//! Example: Get users with ETag caching
//!
//! Run with: cargo run --example users

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

    // Get users (simple)
    println!("--- Get users ---");
    match client.users(node_type, register_id).await {
        Ok(users) => {
            println!("Got {} users", users.len());
            for user in users.iter().take(5) {
                println!("  User ID: {}, UUID: {}", user.id, user.uuid);
            }
            if users.len() > 5 {
                println!("  ... and {} more", users.len() - 5);
            }
        }
        Err(ApiError::NotModified { .. }) => {
            println!("Users not modified (304)");
        }
        Err(e) => println!("Failed to get users: {}", e),
    }

    // Get users with ETag info
    println!("\n--- Get users with ETag ---");
    match client.users_with_etag(node_type, register_id).await {
        Ok(response) => {
            println!("Got {} users", response.data.len());
            if let Some(etag) = response.etag {
                println!("ETag: {}", etag);
            }
        }
        Err(ApiError::NotModified { .. }) => {
            println!("Users not modified (304)");
        }
        Err(e) => println!("Failed to get users: {}", e),
    }

    // Get raw users data
    println!("\n--- Get raw users ---");
    match client.raw_users(node_type, register_id).await {
        Ok(bytes) => {
            let json_str = String::from_utf8_lossy(&bytes);
            println!("Raw users: {}", json_str);
        }
        Err(ApiError::NotModified { .. }) => {
            println!("Users not modified (304)");
        }
        Err(e) => println!("Failed to get raw users: {}", e),
    }

    // Check cached ETag
    println!("\n--- Check cached ETag ---");
    if let Some(etag) = client.get_etag(node_type, register_id).await {
        println!("Cached ETag: {}", etag);
    } else {
        println!("No cached ETag");
    }

    // Clear ETag cache
    println!("\n--- Clear ETag cache ---");
    client.clear_etag_cache().await;
    println!("ETag cache cleared");

    Ok(())
}
