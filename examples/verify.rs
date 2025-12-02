//! Example: Verify registration
//!
//! Run with: cargo run --example verify

use server_r_client::{ApiClient, Config, NodeType};
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

    // Verify valid registration
    println!("--- Verify valid registration ---");
    let valid_register_id = "valid-register-id";
    match client.verify(node_type, valid_register_id).await {
        Ok(valid) => {
            if valid {
                println!("Registration '{}' is valid", valid_register_id);
            } else {
                println!("Registration '{}' is invalid", valid_register_id);
            }
        }
        Err(e) => println!("Verification failed: {}", e),
    }

    // Verify invalid registration
    println!("\n--- Verify invalid registration ---");
    let invalid_register_id = "invalid-register-id";
    match client.verify(node_type, invalid_register_id).await {
        Ok(valid) => {
            if valid {
                println!("Registration '{}' is valid", invalid_register_id);
            } else {
                println!("Registration '{}' is invalid", invalid_register_id);
            }
        }
        Err(e) => println!("Verification failed: {}", e),
    }

    Ok(())
}
