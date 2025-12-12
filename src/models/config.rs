use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::error::{ApiError, Result};
use crate::models::NodeType;

/// Deserialize a boolean that might come as an integer (0/1)
fn bool_from_int<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrInt {
        Bool(bool),
        Int(i64),
    }

    match BoolOrInt::deserialize(deserializer)? {
        BoolOrInt::Bool(b) => Ok(b),
        BoolOrInt::Int(i) => Ok(i != 0),
    }
}

/// Deserialize an optional boolean that might come as an integer (0/1)
#[allow(dead_code)]
fn option_bool_from_int<'de, D>(deserializer: D) -> std::result::Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrInt {
        Bool(bool),
        Int(i64),
    }

    Option::<BoolOrInt>::deserialize(deserializer).map(|opt| {
        opt.map(|v| match v {
            BoolOrInt::Bool(b) => b,
            BoolOrInt::Int(i) => i != 0,
        })
    })
}

/// Base trait for all node configurations
pub trait NodeConfig: Send + Sync {
    /// Get the node type name
    fn type_name(&self) -> &'static str;

    /// Get the node ID
    fn id(&self) -> i64;

    /// Get the server port
    fn server_port(&self) -> u16;
}

/// Trojan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub server_name: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub websocket_config: Option<WebSocketConfig>,
    #[serde(default)]
    pub grpc_config: Option<GrpcConfig>,
}

impl NodeConfig for TrojanConfig {
    fn type_name(&self) -> &'static str {
        "trojan"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// ShadowSocks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowsocksConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
}

impl NodeConfig for ShadowsocksConfig {
    fn type_name(&self) -> &'static str {
        "shadowsocks"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// Hysteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HysteriaConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub obfs: Option<String>,
    #[serde(default)]
    pub up_mbps: Option<i32>,
    #[serde(default)]
    pub down_mbps: Option<i32>,
    #[serde(default)]
    pub disable_mtu_discovery: bool,
    #[serde(default)]
    pub disable_udp: bool,
}

impl NodeConfig for HysteriaConfig {
    fn type_name(&self) -> &'static str {
        "hysteria"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// Hysteria2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hysteria2Config {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub obfs: Option<String>,
    #[serde(default)]
    pub up_mbps: Option<i32>,
    #[serde(default)]
    pub down_mbps: Option<i32>,
    #[serde(default)]
    pub ignore_cli_bandwidth: bool,
    #[serde(default)]
    pub disable_udp: bool,
}

impl NodeConfig for Hysteria2Config {
    fn type_name(&self) -> &'static str {
        "hysteria2"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// VMess configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMessConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub tls: bool,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub tls_config: Option<TlsConfig>,
    #[serde(default)]
    pub websocket_config: Option<WebSocketConfig>,
    #[serde(default)]
    pub h2_config: Option<HttpConfig>,
    #[serde(default)]
    pub tcp_config: Option<TcpConfig>,
    #[serde(default)]
    pub grpc_config: Option<GrpcConfig>,
    #[serde(default)]
    pub router_settings: Option<RouterConfig>,
    #[serde(default)]
    pub dns_settings: Option<DnsConfig>,
}

impl NodeConfig for VMessConfig {
    fn type_name(&self) -> &'static str {
        "vmess"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// AnyTLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyTLSConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub server_name: Option<String>,
    #[serde(default)]
    pub padding_rules: Option<Vec<String>>,
}

impl NodeConfig for AnyTLSConfig {
    fn type_name(&self) -> &'static str {
        "anytls"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// TUIC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuicConfig {
    pub id: i64,
    pub server_port: u16,
    #[serde(default)]
    pub server_name: Option<String>,
    #[serde(default, deserialize_with = "bool_from_int")]
    pub zero_rtt_handshake: bool,
}

impl NodeConfig for TuicConfig {
    fn type_name(&self) -> &'static str {
        "tuic"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn server_port(&self) -> u16 {
        self.server_port
    }
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    #[serde(default)]
    pub server_name: Option<String>,
    #[serde(default)]
    pub certificate: Option<String>,
    #[serde(default)]
    pub private_key: Option<String>,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebSocketConfig {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
}

/// HTTP/2 configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpConfig {
    #[serde(default)]
    pub host: Option<Vec<String>>,
    #[serde(default)]
    pub path: Option<String>,
}

/// TCP configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TcpConfig {
    #[serde(default)]
    pub header: Option<TcpHeader>,
}

/// TCP header configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TcpHeader {
    #[serde(default, rename = "type")]
    pub header_type: Option<String>,
    #[serde(default)]
    pub request: Option<TcpHeaderRequest>,
    #[serde(default)]
    pub response: Option<TcpHeaderResponse>,
}

/// TCP header request configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TcpHeaderRequest {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

/// TCP header response configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TcpHeaderResponse {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

/// gRPC configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrpcConfig {
    #[serde(default)]
    pub service_name: Option<String>,
}

/// Router configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    #[serde(default)]
    pub rules: Option<Vec<RouterRule>>,
}

/// Router rule
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterRule {
    #[serde(default, rename = "type")]
    pub rule_type: Option<String>,
    #[serde(default)]
    pub domain: Option<Vec<String>>,
    #[serde(default)]
    pub ip: Option<Vec<String>>,
    #[serde(default)]
    pub outbound_tag: Option<String>,
}

/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DnsConfig {
    #[serde(default)]
    pub servers: Option<Vec<DnsServer>>,
}

/// DNS server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsServer {
    Simple(String),
    Complex {
        address: String,
        #[serde(default)]
        port: Option<u16>,
        #[serde(default)]
        domains: Option<Vec<String>>,
    },
}

/// Enum wrapper for different node configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum NodeConfigEnum {
    Trojan(TrojanConfig),
    ShadowSocks(ShadowsocksConfig),
    Hysteria(HysteriaConfig),
    Hysteria2(Hysteria2Config),
    VMess(VMessConfig),
    AnyTLS(AnyTLSConfig),
    Tuic(TuicConfig),
}

impl NodeConfigEnum {
    /// Try to convert to TrojanConfig
    pub fn as_trojan(&self) -> Result<&TrojanConfig> {
        match self {
            NodeConfigEnum::Trojan(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "TrojanConfig",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to ShadowsocksConfig
    pub fn as_shadowsocks(&self) -> Result<&ShadowsocksConfig> {
        match self {
            NodeConfigEnum::ShadowSocks(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "ShadowsocksConfig",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to HysteriaConfig
    pub fn as_hysteria(&self) -> Result<&HysteriaConfig> {
        match self {
            NodeConfigEnum::Hysteria(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "HysteriaConfig",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to Hysteria2Config
    pub fn as_hysteria2(&self) -> Result<&Hysteria2Config> {
        match self {
            NodeConfigEnum::Hysteria2(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "Hysteria2Config",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to VMessConfig
    pub fn as_vmess(&self) -> Result<&VMessConfig> {
        match self {
            NodeConfigEnum::VMess(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "VMessConfig",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to AnyTLSConfig
    pub fn as_anytls(&self) -> Result<&AnyTLSConfig> {
        match self {
            NodeConfigEnum::AnyTLS(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "AnyTLSConfig",
                self.type_name(),
            )),
        }
    }

    /// Try to convert to TuicConfig
    pub fn as_tuic(&self) -> Result<&TuicConfig> {
        match self {
            NodeConfigEnum::Tuic(config) => Ok(config),
            _ => Err(ApiError::type_conversion_error(
                "TuicConfig",
                self.type_name(),
            )),
        }
    }

    /// Get the type name
    pub fn type_name(&self) -> &'static str {
        match self {
            NodeConfigEnum::Trojan(_) => "trojan",
            NodeConfigEnum::ShadowSocks(_) => "shadowsocks",
            NodeConfigEnum::Hysteria(_) => "hysteria",
            NodeConfigEnum::Hysteria2(_) => "hysteria2",
            NodeConfigEnum::VMess(_) => "vmess",
            NodeConfigEnum::AnyTLS(_) => "anytls",
            NodeConfigEnum::Tuic(_) => "tuic",
        }
    }
}

/// Parse configuration based on node type
pub fn parse_config(node_type: NodeType, data: &[u8]) -> Result<NodeConfigEnum> {
    let config = match node_type {
        NodeType::Trojan => {
            let config: TrojanConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::Trojan(config)
        }
        NodeType::ShadowSocks => {
            let config: ShadowsocksConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::ShadowSocks(config)
        }
        NodeType::Hysteria => {
            let config: HysteriaConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::Hysteria(config)
        }
        NodeType::Hysteria2 => {
            let config: Hysteria2Config = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::Hysteria2(config)
        }
        NodeType::VMess => {
            let config: VMessConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::VMess(config)
        }
        NodeType::AnyTLS => {
            let config: AnyTLSConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::AnyTLS(config)
        }
        NodeType::Tuic => {
            let config: TuicConfig = serde_json::from_slice(data)
                .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;
            NodeConfigEnum::Tuic(config)
        }
    };
    Ok(config)
}

/// Parse configuration from API response with `{"data": ...}` wrapper
///
/// This function handles the standard API response format where the actual
/// config is wrapped in a `data` field. Use this for parsing raw API responses.
///
/// For parsing unwrapped config data, use [`parse_config`] instead.
pub fn parse_raw_config_response(node_type: NodeType, data: &[u8]) -> Result<NodeConfigEnum> {
    use crate::models::ApiResponse;

    let api_response: ApiResponse<serde_json::Value> = serde_json::from_slice(data)
        .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;

    let config_bytes = serde_json::to_vec(&api_response.data)
        .map_err(|e| ApiError::parse_error(e.to_string(), "", Some(e)))?;

    parse_config(node_type, &config_bytes)
}
