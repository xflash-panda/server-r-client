//! Example: Get node configuration
//!
//! Run with: cargo run --example config

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
    let node_id = 1;

    // Get parsed configuration
    println!("--- Get parsed config ---");
    match client.config(node_type, node_id).await {
        Ok(config) => {
            println!("Node type: {}", config.type_name());

            // Type-safe access to specific config
            match config.as_trojan() {
                Ok(trojan) => {
                    println!("  ID: {}", trojan.id);
                    println!("  Server port: {}", trojan.server_port);
                    println!("  Allow insecure: {}", trojan.allow_insecure);
                    if let Some(ref name) = trojan.server_name {
                        println!("  Server name: {}", name);
                    }
                }
                Err(e) => println!("Not a Trojan config: {}", e),
            }
        }
        Err(e) => println!("Failed to get config: {}", e),
    }

    // Get raw configuration (bytes)
    println!("\n--- Get raw config ---");
    match client.raw_config(node_type, node_id).await {
        Ok(bytes) => {
            let json_str = String::from_utf8_lossy(&bytes);
            println!("Raw config: {}", json_str);
        }
        Err(e) => println!("Failed to get raw config: {}", e),
    }

    Ok(())
}
