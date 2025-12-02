//! Example: Submit traffic data
//!
//! Run with: cargo run --example submit

use server_r_client::{ApiClient, Config, NodeType, TrafficStats, UserTraffic};
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

    // Submit user traffic
    println!("--- Submit traffic ---");
    let traffic_data = vec![
        UserTraffic::new(1, 1024 * 1024, 2048 * 1024), // 1MB up, 2MB down
        UserTraffic::new(2, 512 * 1024, 1024 * 1024),  // 512KB up, 1MB down
        UserTraffic::with_count(3, 256 * 1024, 512 * 1024, 10), // with connection count
    ];

    match client.submit(node_type, register_id, traffic_data).await {
        Ok(()) => println!("Traffic submitted successfully!"),
        Err(e) => println!("Failed to submit traffic: {}", e),
    }

    // Submit traffic with agent info
    println!("\n--- Submit traffic with agent ---");
    let traffic_data = vec![
        UserTraffic::new(1, 100 * 1024, 200 * 1024),
        UserTraffic::new(2, 50 * 1024, 100 * 1024),
    ];

    match client
        .submit_with_agent(node_type, register_id, traffic_data)
        .await
    {
        Ok(()) => println!("Traffic with agent submitted successfully!"),
        Err(e) => println!("Failed to submit traffic with agent: {}", e),
    }

    // Submit aggregated statistics
    println!("\n--- Submit traffic stats ---");
    let mut stats = TrafficStats::new();
    stats.add_user(1, 100); // user 1: 100 requests
    stats.add_user(2, 50); // user 2: 50 requests
    stats.add_user(3, 200); // user 3: 200 requests

    println!("Stats to submit:");
    println!("  Total count: {}", stats.count);
    println!("  Total requests: {}", stats.requests);
    println!("  User IDs: {:?}", stats.user_ids);

    match client
        .submit_stats_with_agent(node_type, register_id, stats)
        .await
    {
        Ok(()) => println!("Traffic stats submitted successfully!"),
        Err(e) => println!("Failed to submit traffic stats: {}", e),
    }

    Ok(())
}
