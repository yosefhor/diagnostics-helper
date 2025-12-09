mod api;
mod config;
mod models;
mod snapshot;

use crate::config::load_config;
use crate::snapshot::SnapshotManager;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting diagnostics-helper...");

    // Load config
    let cfg = load_config("config/config.json")?;
    tracing::info!("Loaded config: {:?}", cfg);

    // Create manager and load existing snapshots
    let manager = Arc::new(SnapshotManager::new(cfg.snapshots_file.clone()));
    manager.load_from_file().await?;
    tracing::info!("Snapshots loaded.");

    // Optionally spawn background collector
    if cfg.enabled {
        let m = Arc::clone(&manager);
        let interval = cfg.collect_interval_ms;
        tokio::spawn(async move {
            loop {
                // build fake snapshot
                let snap = crate::models::Snapshot {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: chrono::Utc::now().timestamp_millis() as u128,
                    message: format!("Auto snapshot"),
                    status: "green".to_string(),
                };

                if let Err(e) = m.add_snapshot(snap).await {
                    tracing::error!("background add_snapshot error: {:?}", e);
                } else {
                    tracing::info!("background snapshot saved");
                }

                sleep(Duration::from_millis(interval)).await;
            }
        });
    }

    // Start HTTP server
    let app = api::create_router(Arc::clone(&manager));

    let addr = cfg
        .listen_addr
        .parse::<std::net::SocketAddr>()
        .expect("invalid listen_addr");
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(listener, app).await?;

    Ok(())
}
