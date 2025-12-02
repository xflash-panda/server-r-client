use serde::{Deserialize, Serialize};

use super::user::{TrafficStats, UserTraffic};

/// Node registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub hostname: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
}

impl RegisterRequest {
    pub fn new(hostname: impl Into<String>, port: u16) -> Self {
        Self {
            hostname: hostname.into(),
            port,
            node_ip: None,
        }
    }

    pub fn with_node_ip(mut self, node_ip: impl Into<String>) -> Self {
        self.node_ip = Some(node_ip.into());
        self
    }
}

/// Verify request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub register_id: String,
}

impl VerifyRequest {
    pub fn new(register_id: impl Into<String>) -> Self {
        Self {
            register_id: register_id.into(),
        }
    }
}

/// Heartbeat request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub register_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
}

impl HeartbeatRequest {
    pub fn new(register_id: impl Into<String>) -> Self {
        Self {
            register_id: register_id.into(),
            node_ip: None,
        }
    }

    pub fn with_node_ip(mut self, node_ip: impl Into<String>) -> Self {
        self.node_ip = Some(node_ip.into());
        self
    }
}

/// Traffic submission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitRequest {
    pub register_id: String,
    pub data: Vec<UserTraffic>,
}

impl SubmitRequest {
    pub fn new(register_id: impl Into<String>, data: Vec<UserTraffic>) -> Self {
        Self {
            register_id: register_id.into(),
            data,
        }
    }
}

/// Traffic stats submission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitStatsRequest {
    pub register_id: String,
    pub data: TrafficStats,
}

impl SubmitStatsRequest {
    pub fn new(register_id: impl Into<String>, data: TrafficStats) -> Self {
        Self {
            register_id: register_id.into(),
            data,
        }
    }
}
