use crate::models::Snapshot;
use serde_json::{from_str, to_string_pretty};
use std::sync::Arc;
use thiserror::Error;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum SnapshotError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct SnapshotManager {
    // Protected by async RwLock for concurrent access
    inner: Arc<RwLock<Vec<Snapshot>>>,
    file_path: String,
}

impl SnapshotManager {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Vec::new())),
            file_path: file_path.into(),
        }
    }

    // load snapshots from file into memory
    pub async fn load_from_file(&self) -> Result<(), SnapshotError> {
        // if file doesn't exist, create empty file
        if let Err(e) = fs::metadata(&self.file_path).await {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir_all(
                    std::path::Path::new(&self.file_path)
                        .parent()
                        .unwrap_or(std::path::Path::new("data")),
                )
                .await
                .ok();
                fs::write(&self.file_path, "[]").await?;
            } else {
                return Err(SnapshotError::Io(e));
            }
        }

        let raw = fs::read_to_string(&self.file_path).await?;
        let list: Vec<Snapshot> = match from_str(&raw) {
            Ok(v) => v,
            Err(_) => Vec::new(), // fallback if corrupted / empty
        };
        let mut lock = self.inner.write().await;
        *lock = list;
        Ok(())
    }

    // save entire memory to file (overwrite)
    pub async fn save_to_file(&self) -> Result<(), SnapshotError> {
        let lock = self.inner.read().await;
        let json = to_string_pretty(&*lock)?;
        fs::write(&self.file_path, json).await?;
        Ok(())
    }

    // append snapshot to memory and persist
    pub async fn add_snapshot(&self, s: Snapshot) -> Result<(), SnapshotError> {
        {
            let mut lock = self.inner.write().await;
            lock.push(s);
        }
        // Persist entire file (simple approach)
        self.save_to_file().await?;
        Ok(())
    }

    // list in-memory snapshots
    pub async fn list_snapshots(&self) -> Vec<Snapshot> {
        let lock = self.inner.read().await;
        lock.clone()
    }
}
