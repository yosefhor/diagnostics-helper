use crate::models::Snapshot;
use crate::snapshot::SnapshotManager;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct NewSnapshot {
    pub message: String,
    pub status: String,
}

pub fn create_router(manager: Arc<SnapshotManager>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/snapshots", get(get_snapshots).post(post_snapshot))
        .with_state(manager)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}

async fn get_snapshots(State(manager): State<Arc<SnapshotManager>>) -> Json<Vec<Snapshot>> {
    let list = manager.list_snapshots().await;
    Json(list)
}

async fn post_snapshot(
    State(manager): State<Arc<SnapshotManager>>,
    Json(payload): Json<NewSnapshot>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Build snapshot
    let snap = Snapshot {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().timestamp_millis() as u128,
        message: payload.message,
        status: payload.status,
    };

    if let Err(e) = manager.add_snapshot(snap).await {
        tracing::error!("failed to add snapshot: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "failed to persist snapshot" })),
        );
    }

    (StatusCode::CREATED, Json(serde_json::json!({ "ok": true })))
}
