use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported proxy node types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Trojan,
    #[serde(rename = "shadowsocks")]
    ShadowSocks,
    Hysteria,
    Hysteria2,
    #[serde(rename = "vmess")]
    VMess,
    #[serde(rename = "anytls")]
    AnyTLS,
}

impl NodeType {
    /// Get the URL path segment for this node type
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Trojan => "trojan",
            NodeType::ShadowSocks => "shadowsocks",
            NodeType::Hysteria => "hysteria",
            NodeType::Hysteria2 => "hysteria2",
            NodeType::VMess => "vmess",
            NodeType::AnyTLS => "anytls",
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for NodeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trojan" => Ok(NodeType::Trojan),
            "shadowsocks" | "ss" => Ok(NodeType::ShadowSocks),
            "hysteria" => Ok(NodeType::Hysteria),
            "hysteria2" => Ok(NodeType::Hysteria2),
            "vmess" => Ok(NodeType::VMess),
            "anytls" => Ok(NodeType::AnyTLS),
            _ => Err(format!("Unknown node type: {}", s)),
        }
    }
}
