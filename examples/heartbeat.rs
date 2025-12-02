//! Example: Send heartbeat
//!
//! Run with: cargo run --example heartbeat

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
    let register_id = "your-register-id";

    // Simple heartbeat
    println!("--- Send heartbeat ---");
    match client.heartbeat(node_type, register_id).await {
        Ok(()) => println!("Heartbeat sent successfully!"),
        Err(e) => println!("Failed to send heartbeat: {}", e),
    }

    // Heartbeat with node IP
    println!("\n--- Send heartbeat with IP ---");
    match client
        .heartbeat_with_ip(node_type, register_id, "1.2.3.4")
        .await
    {
        Ok(()) => println!("Heartbeat with IP sent successfully!"),
        Err(e) => println!("Failed to send heartbeat with IP: {}", e),
    }

    // Periodic heartbeat example
    println!("\n--- Periodic heartbeat (3 times) ---");
    for i in 1..=3 {
        match client.heartbeat(node_type, register_id).await {
            Ok(()) => println!("Heartbeat #{} sent", i),
            Err(e) => println!("Heartbeat #{} failed: {}", i, e),
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
