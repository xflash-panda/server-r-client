//! Example: Working with different node types
//!
//! Run with: cargo run --example all_node_types

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

    // All supported node types
    let node_types = [
        NodeType::Trojan,
        NodeType::ShadowSocks,
        NodeType::Hysteria,
        NodeType::Hysteria2,
        NodeType::VMess,
        NodeType::AnyTLS,
        NodeType::Tuic,
    ];

    println!("--- Supported node types ---\n");

    for node_type in &node_types {
        println!("Node type: {} ({})", node_type, node_type.as_str());
    }

    // Parse node types from string
    println!("\n--- Parse node types from string ---\n");

    let strings = [
        "trojan",
        "shadowsocks",
        "ss",
        "hysteria",
        "hysteria2",
        "vmess",
        "anytls",
        "tuic",
    ];
    for s in &strings {
        match s.parse::<NodeType>() {
            Ok(nt) => println!("'{}' -> {:?}", s, nt),
            Err(e) => println!("'{}' -> Error: {}", s, e),
        }
    }

    // Get config for each node type
    println!("\n--- Get config for each node type ---\n");

    for node_type in &node_types {
        print!("{}: ", node_type);
        match client.config(*node_type, 1).await {
            Ok(config) => {
                println!("OK ({})", config.type_name());

                // Type-specific handling
                match node_type {
                    NodeType::Trojan => {
                        if let Ok(c) = config.as_trojan() {
                            println!("  Port: {}, ServerName: {:?}", c.server_port, c.server_name);
                        }
                    }
                    NodeType::ShadowSocks => {
                        if let Ok(c) = config.as_shadowsocks() {
                            println!("  Port: {}, Method: {:?}", c.server_port, c.method);
                        }
                    }
                    NodeType::Hysteria => {
                        if let Ok(c) = config.as_hysteria() {
                            println!(
                                "  Port: {}, Up: {:?}Mbps, Down: {:?}Mbps",
                                c.server_port, c.up_mbps, c.down_mbps
                            );
                        }
                    }
                    NodeType::Hysteria2 => {
                        if let Ok(c) = config.as_hysteria2() {
                            println!(
                                "  Port: {}, IgnoreCliBandwidth: {}",
                                c.server_port, c.ignore_cli_bandwidth
                            );
                        }
                    }
                    NodeType::VMess => {
                        if let Ok(c) = config.as_vmess() {
                            println!(
                                "  Port: {}, TLS: {}, Network: {:?}",
                                c.server_port, c.tls, c.network
                            );
                        }
                    }
                    NodeType::AnyTLS => {
                        if let Ok(c) = config.as_anytls() {
                            println!("  Port: {}, ServerName: {:?}", c.server_port, c.server_name);
                        }
                    }
                    NodeType::Tuic => {
                        if let Ok(c) = config.as_tuic() {
                            println!(
                                "  Port: {}, ServerName: {:?}, ZeroRttHandshake: {}",
                                c.server_port, c.server_name, c.zero_rtt_handshake
                            );
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    // Register with different node types
    println!("\n--- Register with different node types ---\n");

    for node_type in &node_types {
        let request = RegisterRequest::new("node.example.com", 443);
        print!("{}: ", node_type);
        match client.register(*node_type, 1, request).await {
            Ok(register_id) => println!("Registered (ID: {})", register_id),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
