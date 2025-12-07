use axum::{
    extract::{Path, Json, State},
    response::IntoResponse,
    http::StatusCode,
};
use crate::db::Db;
use crate::models::Holder;

pub async fn create_holder(
    State(db): State<Db>,
    Json(payload): Json<Holder>,
) -> impl IntoResponse {
    let mut map = db.lock().await;
    map.insert(payload.id.clone(), payload);
    StatusCode::CREATED
}

pub async fn read_holder(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let map = db.lock().await;
    if let Some(h) = map.get(&id) {
        Json(h.clone()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn update_holder(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(payload): Json<Holder>,
) -> impl IntoResponse {
    let mut map = db.lock().await;
    if let Some(h) = map.get_mut(&id) {
        h.value = payload.value;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_holder(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut map = db.lock().await;
    if map.remove(&id).is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
