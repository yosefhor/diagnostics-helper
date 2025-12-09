use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: u128, // milliseconds since epoch
    pub message: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub collect_interval_ms: u64,
    pub enabled: bool,
    pub listen_addr: String,
    pub snapshots_file: String,
}
