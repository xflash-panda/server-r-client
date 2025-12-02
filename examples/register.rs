//! Example: Register and unregister a node
//!
//! Run with: cargo run --example register

use server_r_client::{ApiClient, Config, NodeType, RegisterRequest};
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

    // Register a node
    println!("--- Register node ---");
    let register_request = RegisterRequest::new("node.example.com", 443);

    match client.register(node_type, node_id, register_request).await {
        Ok(register_id) => {
            println!("Registered successfully!");
            println!("Register ID: {}", register_id);

            // Verify the registration
            println!("\n--- Verify registration ---");
            match client.verify(node_type, &register_id).await {
                Ok(valid) => println!("Registration valid: {}", valid),
                Err(e) => println!("Verify failed: {}", e),
            }

            // Unregister the node
            println!("\n--- Unregister node ---");
            match client.unregister(node_type, &register_id).await {
                Ok(()) => println!("Unregistered successfully!"),
                Err(e) => println!("Unregister failed: {}", e),
            }
        }
        Err(e) => println!("Registration failed: {}", e),
    }

    // Register with node IP
    println!("\n--- Register with node IP ---");
    let register_request_with_ip =
        RegisterRequest::new("node.example.com", 443).with_node_ip("1.2.3.4");

    match client
        .register(node_type, node_id, register_request_with_ip)
        .await
    {
        Ok(register_id) => {
            println!("Registered with IP!");
            println!("Register ID: {}", register_id);
        }
        Err(e) => println!("Registration failed: {}", e),
    }

    Ok(())
}
