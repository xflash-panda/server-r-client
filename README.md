# server-r-client

Rust HTTP client library for xflash-panda server communication.

This library provides a type-safe API client for communicating with a centralized proxy server management panel. It supports various proxy protocols including Trojan, ShadowSocks, Hysteria, Hysteria2, VMess, AnyTLS, and Tuic.

## Features

- Node lifecycle management (registration, verification, unregistration)
- Configuration retrieval for various proxy protocols
- User list management with ETag caching
- Traffic statistics reporting
- Heartbeat/health check functionality

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
server-r-client = "0.1.0"
```

## Quick Start

```rust
use server_r_client::{ApiClient, Config, NodeType, RegisterRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration
    let config = Config::new("https://api.example.com", "your-api-token")
        .with_timeout(Duration::from_secs(10))
        .with_debug(true);

    // Create the API client
    let client = ApiClient::new(config)?;

    // Register a node
    let register_id = client
        .register(
            NodeType::Trojan,
            1,
            RegisterRequest::new("node.example.com", 443),
        )
        .await?;

    println!("Registered with ID: {}", register_id);

    // Get users
    let users = client.users(NodeType::Trojan, &register_id).await?;
    println!("Got {} users", users.len());

    // Send heartbeat
    client.heartbeat(NodeType::Trojan, &register_id).await?;

    Ok(())
}
```

## Supported Node Types

- `NodeType::Trojan` - Trojan protocol
- `NodeType::ShadowSocks` - ShadowSocks protocol
- `NodeType::Hysteria` - Hysteria protocol
- `NodeType::Hysteria2` - Hysteria2 protocol
- `NodeType::VMess` - VMess protocol
- `NodeType::AnyTLS` - AnyTLS protocol
- `NodeType::Tuic` - Tuic protocol

## API Methods

| Method | Description |
|--------|-------------|
| `config()` | Get node configuration |
| `register()` | Register a node |
| `verify()` | Verify registration status |
| `unregister()` | Unregister a node |
| `users()` | Get user list (with ETag caching) |
| `submit()` | Submit traffic statistics |
| `heartbeat()` | Send heartbeat |

## Error Handling

The library provides detailed error types through the `ApiError` enum:

```rust
use server_r_client::{ApiClient, NodeType, ApiError};

async fn example(client: &ApiClient, register_id: &str) {
    match client.users(NodeType::Trojan, register_id).await {
        Ok(users) => println!("Got {} users", users.len()),
        Err(ApiError::NotModified { .. }) => println!("Users not modified"),
        Err(ApiError::ServerError { status_code, message, .. }) => {
            println!("Server error {}: {}", status_code, message);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

### Error Types

- `ServerError` - HTTP 4xx/5xx errors
- `NetworkError` - Connection/network failures
- `ParseError` - JSON parsing failures
- `NotModified` - HTTP 304 (useful for ETag caching)

## Examples

Run examples with:

```bash
# Basic usage
cargo run --example basic_usage

# Node type specific examples
cargo run --example config
cargo run --example register
cargo run --example users
cargo run --example submit
cargo run --example heartbeat
cargo run --example verify
cargo run --example error_handling
cargo run --example all_node_types
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `API_HOST` | API server URL |
| `API_TOKEN` | Authentication token |

## License

MIT
