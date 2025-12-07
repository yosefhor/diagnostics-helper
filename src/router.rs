use axum::{routing::{get, post, put, delete}, Router};
use crate::db::Db;
use crate::handlers::*;

pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/holders", post(create_holder))
        .route("/holders/{id}", get(read_holder))
        .route("/holders/{id}", put(update_holder))
        .route("/holders/{id}", delete(delete_holder))
        .with_state(db)
}