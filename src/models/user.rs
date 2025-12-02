use serde::{Deserialize, Serialize};

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub uuid: String,
}

/// User traffic data for submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTraffic {
    pub user_id: i64,
    /// Upload bytes
    pub u: u64,
    /// Download bytes
    pub d: u64,
    /// Count/connections
    #[serde(default)]
    pub n: u64,
}

impl UserTraffic {
    /// Create a new UserTraffic instance
    pub fn new(user_id: i64, upload: u64, download: u64) -> Self {
        Self {
            user_id,
            u: upload,
            d: download,
            n: 0,
        }
    }

    /// Create a new UserTraffic instance with connection count
    pub fn with_count(user_id: i64, upload: u64, download: u64, count: u64) -> Self {
        Self {
            user_id,
            u: upload,
            d: download,
            n: count,
        }
    }
}

/// Aggregated traffic statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficStats {
    /// Total count
    pub count: i64,
    /// Total requests
    pub requests: i64,
    /// User IDs
    pub user_ids: Vec<i64>,
    /// Per-user request counts
    #[serde(default)]
    pub user_requests: std::collections::HashMap<i64, i64>,
}

impl TrafficStats {
    /// Create a new empty TrafficStats instance
    pub fn new() -> Self {
        Self {
            count: 0,
            requests: 0,
            user_ids: Vec::new(),
            user_requests: std::collections::HashMap::new(),
        }
    }

    /// Add a user's request count
    pub fn add_user(&mut self, user_id: i64, requests: i64) {
        self.user_ids.push(user_id);
        self.user_requests.insert(user_id, requests);
        self.requests += requests;
        self.count += 1;
    }
}

impl Default for TrafficStats {
    fn default() -> Self {
        Self::new()
    }
}
