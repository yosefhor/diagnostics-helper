use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: u128, // milliseconds since epoch
    pub message: String,
    pub status: String,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<(u64, u64)>, // (used_mb, total_mb)
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub collect_interval_ms: u64,
    pub enabled: bool,
    pub listen_addr: String,
    pub snapshots_file: String,
}
