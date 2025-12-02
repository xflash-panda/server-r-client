use server_r_client::{
    ApiClient, ApiError, Config, NodeType, RegisterRequest, TrafficStats, UserTraffic,
    TrojanConfig, ShadowsocksConfig, HysteriaConfig, Hysteria2Config, VMessConfig, AnyTLSConfig,
    NodeConfigEnum,
};
use std::time::Duration;

// Unit tests that don't require network
#[test]
fn test_node_type_parsing() {
    assert_eq!(NodeType::Trojan.as_str(), "trojan");
    assert_eq!(NodeType::ShadowSocks.as_str(), "shadowsocks");
    assert_eq!(NodeType::Hysteria.as_str(), "hysteria");
    assert_eq!(NodeType::Hysteria2.as_str(), "hysteria2");
    assert_eq!(NodeType::VMess.as_str(), "vmess");
    assert_eq!(NodeType::AnyTLS.as_str(), "anytls");

    assert_eq!("trojan".parse::<NodeType>().unwrap(), NodeType::Trojan);
    assert_eq!(
        "shadowsocks".parse::<NodeType>().unwrap(),
        NodeType::ShadowSocks
    );
    assert_eq!("ss".parse::<NodeType>().unwrap(), NodeType::ShadowSocks);
    assert!("invalid".parse::<NodeType>().is_err());
}

#[test]
fn test_node_type_display() {
    assert_eq!(format!("{}", NodeType::Trojan), "trojan");
    assert_eq!(format!("{}", NodeType::ShadowSocks), "shadowsocks");
    assert_eq!(format!("{}", NodeType::VMess), "vmess");
}

#[test]
fn test_traffic_stats() {
    let mut stats = TrafficStats::new();
    assert_eq!(stats.count, 0);
    assert_eq!(stats.requests, 0);

    stats.add_user(1, 100);
    stats.add_user(2, 200);

    assert_eq!(stats.count, 2);
    assert_eq!(stats.requests, 300);
    assert_eq!(stats.user_ids, vec![1, 2]);
    assert_eq!(stats.user_requests.get(&1), Some(&100));
    assert_eq!(stats.user_requests.get(&2), Some(&200));
}

#[test]
fn test_traffic_stats_default() {
    let stats = TrafficStats::default();
    assert_eq!(stats.count, 0);
    assert_eq!(stats.requests, 0);
    assert!(stats.user_ids.is_empty());
    assert!(stats.user_requests.is_empty());
}

#[test]
fn test_user_traffic_creation() {
    let traffic = UserTraffic::new(1, 1024, 2048);
    assert_eq!(traffic.user_id, 1);
    assert_eq!(traffic.u, 1024);
    assert_eq!(traffic.d, 2048);
    assert_eq!(traffic.n, 0);

    let traffic_with_count = UserTraffic::with_count(2, 512, 1024, 10);
    assert_eq!(traffic_with_count.user_id, 2);
    assert_eq!(traffic_with_count.u, 512);
    assert_eq!(traffic_with_count.d, 1024);
    assert_eq!(traffic_with_count.n, 10);
}

#[test]
fn test_register_request_builder() {
    let request = RegisterRequest::new("node.example.com", 443);
    assert_eq!(request.hostname, "node.example.com");
    assert_eq!(request.port, 443);
    assert!(request.node_ip.is_none());

    let request_with_ip = RegisterRequest::new("node.example.com", 443).with_node_ip("1.2.3.4");
    assert_eq!(request_with_ip.node_ip, Some("1.2.3.4".to_string()));
}

#[test]
fn test_config_creation() {
    let config = Config::new("https://api.example.com", "test-token");
    assert_eq!(config.api_host, "https://api.example.com");
    assert_eq!(config.token, "test-token");
    assert_eq!(config.timeout, Duration::from_secs(5));
    assert!(!config.debug);
}

#[test]
fn test_config_builder() {
    let config = Config::new("https://api.example.com", "test-token")
        .with_timeout(Duration::from_secs(30))
        .with_debug(true);

    assert_eq!(config.timeout, Duration::from_secs(30));
    assert!(config.debug);
}

#[test]
fn test_client_creation() {
    let config = Config::new("https://api.example.com", "test-token");
    let client = ApiClient::new(config);
    assert!(client.is_ok());
}

#[test]
fn test_error_types() {
    let server_err = ApiError::from_status_code(500, "Internal Server Error", "http://test.com");
    assert!(server_err.is_server_error());
    assert!(!server_err.is_network_error());
    assert!(!server_err.is_parse_error());
    assert!(!server_err.is_not_modified());

    let network_err = ApiError::network_error("Connection refused", "http://test.com", None);
    assert!(network_err.is_network_error());
    assert!(!network_err.is_server_error());

    let parse_err = ApiError::parse_error("Invalid JSON", "http://test.com", None);
    assert!(parse_err.is_parse_error());
    assert!(!parse_err.is_server_error());

    let not_modified = ApiError::not_modified("http://test.com");
    assert!(not_modified.is_not_modified());
    assert!(!not_modified.is_server_error());
}

#[test]
fn test_trojan_config_deserialization() {
    let json = r#"{
        "id": 1,
        "server_port": 443,
        "allow_insecure": false,
        "server_name": "example.com"
    }"#;

    let config: TrojanConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 1);
    assert_eq!(config.server_port, 443);
    assert!(!config.allow_insecure);
    assert_eq!(config.server_name, Some("example.com".to_string()));
}

#[test]
fn test_shadowsocks_config_deserialization() {
    let json = r#"{
        "id": 2,
        "server_port": 8388,
        "method": "aes-256-gcm",
        "network": "tcp"
    }"#;

    let config: ShadowsocksConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 2);
    assert_eq!(config.server_port, 8388);
    assert_eq!(config.method, Some("aes-256-gcm".to_string()));
}

#[test]
fn test_hysteria_config_deserialization() {
    let json = r#"{
        "id": 3,
        "server_port": 443,
        "protocol": "udp",
        "obfs": "salamander",
        "up_mbps": 100,
        "down_mbps": 100
    }"#;

    let config: HysteriaConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 3);
    assert_eq!(config.server_port, 443);
    assert_eq!(config.up_mbps, Some(100));
    assert_eq!(config.down_mbps, Some(100));
}

#[test]
fn test_hysteria2_config_deserialization() {
    let json = r#"{
        "id": 4,
        "server_port": 443,
        "obfs": "salamander",
        "ignore_cli_bandwidth": true
    }"#;

    let config: Hysteria2Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 4);
    assert_eq!(config.server_port, 443);
    assert!(config.ignore_cli_bandwidth);
}

#[test]
fn test_vmess_config_deserialization() {
    let json = r#"{
        "id": 5,
        "server_port": 443,
        "tls": true,
        "network": "ws"
    }"#;

    let config: VMessConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 5);
    assert_eq!(config.server_port, 443);
    assert!(config.tls);
    assert_eq!(config.network, Some("ws".to_string()));
}

#[test]
fn test_anytls_config_deserialization() {
    let json = r#"{
        "id": 6,
        "server_port": 443,
        "allow_insecure": false,
        "server_name": "example.com",
        "padding_rules": ["rule1", "rule2"]
    }"#;

    let config: AnyTLSConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.id, 6);
    assert_eq!(config.server_port, 443);
    assert!(!config.allow_insecure);
    assert_eq!(config.padding_rules, Some(vec!["rule1".to_string(), "rule2".to_string()]));
}

#[test]
fn test_node_config_enum_type_conversion() {
    let trojan = NodeConfigEnum::Trojan(TrojanConfig {
        id: 1,
        server_port: 443,
        allow_insecure: false,
        server_name: None,
        network: None,
        websocket_config: None,
        grpc_config: None,
    });

    assert!(trojan.as_trojan().is_ok());
    assert!(trojan.as_shadowsocks().is_err());
    assert!(trojan.as_vmess().is_err());
    assert_eq!(trojan.type_name(), "trojan");
}

#[test]
fn test_user_serialization() {
    use server_r_client::User;

    let user = User {
        id: 1,
        uuid: "test-uuid-123".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"uuid\":\"test-uuid-123\""));

    let deserialized: User = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.uuid, "test-uuid-123");
}

#[test]
fn test_user_traffic_serialization() {
    let traffic = UserTraffic::with_count(1, 1000, 2000, 5);

    let json = serde_json::to_string(&traffic).unwrap();
    assert!(json.contains("\"user_id\":1"));
    assert!(json.contains("\"u\":1000"));
    assert!(json.contains("\"d\":2000"));
    assert!(json.contains("\"n\":5"));
}

#[test]
fn test_traffic_stats_serialization() {
    let mut stats = TrafficStats::new();
    stats.add_user(1, 100);

    let json = serde_json::to_string(&stats).unwrap();
    assert!(json.contains("\"count\":1"));
    assert!(json.contains("\"requests\":100"));
}
