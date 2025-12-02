//! Basic usage example for server-r-client
//!
//! Run with: cargo run --example basic_usage

use server_r_client::{ApiClient, ApiError, Config, NodeType, RegisterRequest, UserTraffic};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging (optional)
    tracing_subscriber::fmt::init();

    // Create client configuration
    let config = Config::new(
        std::env::var("API_HOST").unwrap_or_else(|_| "https://api.example.com".to_string()),
        std::env::var("API_TOKEN").unwrap_or_else(|_| "your-api-token".to_string()),
    )
    .with_timeout(Duration::from_secs(10))
    .with_debug(true);

    // Create the API client
    let client = ApiClient::new(config)?;
    println!("Created API client: {:?}", client);

    // Example: Get node configuration
    let node_type = NodeType::Trojan;
    let node_id = 1;

    println!("\n--- Getting node configuration ---");
    match client.config(node_type, node_id).await {
        Ok(config) => {
            println!("Got config for node type: {}", config.type_name());
            if let Ok(trojan_config) = config.as_trojan() {
                println!("  Server port: {}", trojan_config.server_port);
                println!("  Allow insecure: {}", trojan_config.allow_insecure);
            }
        }
        Err(e) => println!("Failed to get config: {}", e),
    }

    // Example: Register a node
    println!("\n--- Registering node ---");
    let register_request = RegisterRequest::new("node.example.com", 443).with_node_ip("1.2.3.4");

    match client.register(node_type, node_id, register_request).await {
        Ok(register_id) => {
            println!("Registered successfully! ID: {}", register_id);

            // Example: Get users
            println!("\n--- Getting users ---");
            match client.users(node_type, &register_id).await {
                Ok(users) => {
                    println!("Got {} users", users.len());
                    for user in users.iter().take(5) {
                        println!("  User {}: {}", user.id, user.uuid);
                    }
                }
                Err(ApiError::NotModified { .. }) => {
                    println!("Users not modified (304)");
                }
                Err(e) => println!("Failed to get users: {}", e),
            }

            // Example: Submit traffic data
            println!("\n--- Submitting traffic ---");
            let traffic_data = vec![
                UserTraffic::new(1, 1024 * 1024, 2048 * 1024), // 1MB up, 2MB down
                UserTraffic::with_count(2, 512 * 1024, 1024 * 1024, 10), // with connection count
            ];

            match client.submit(node_type, &register_id, traffic_data).await {
                Ok(()) => println!("Traffic submitted successfully"),
                Err(e) => println!("Failed to submit traffic: {}", e),
            }

            // Example: Send heartbeat
            println!("\n--- Sending heartbeat ---");
            match client.heartbeat(node_type, &register_id).await {
                Ok(()) => println!("Heartbeat sent successfully"),
                Err(e) => println!("Failed to send heartbeat: {}", e),
            }

            // Example: Verify registration
            println!("\n--- Verifying registration ---");
            match client.verify(node_type, &register_id).await {
                Ok(valid) => println!("Registration valid: {}", valid),
                Err(e) => println!("Failed to verify: {}", e),
            }

            // Example: Unregister
            println!("\n--- Unregistering ---");
            match client.unregister(node_type, &register_id).await {
                Ok(()) => println!("Unregistered successfully"),
                Err(e) => println!("Failed to unregister: {}", e),
            }
        }
        Err(e) => println!("Failed to register: {}", e),
    }

    Ok(())
}
