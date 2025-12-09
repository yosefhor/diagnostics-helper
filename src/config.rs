use crate::models::AppConfig;
use serde_json::from_str;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub fn load_config(path: &str) -> Result<AppConfig, ConfigError> {
    let raw = fs::read_to_string(path)?;
    let cfg: AppConfig = from_str(&raw)?;
    // Validate
    if cfg.collect_interval_ms == 0 {
        return Err(ConfigError::Validation(
            "collect_interval_ms must be > 0".to_string(),
        ));
    }
    Ok(cfg)
}
