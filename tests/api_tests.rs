use http_body_util::BodyExt;
use serde_json::Value;
use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use diagnostics_helper::api::create_router;
use diagnostics_helper::snapshot::SnapshotManager;

fn test_app() -> axum::Router {
    let manager = Arc::new(SnapshotManager::new("test_snapshots.json"));
    create_router(manager)
}

#[tokio::test]
async fn post_event_then_status_contains_it() {
    let app = test_app();

    let body = r#"{ "message": "hi", "status": "for_test" }"#;

    let post_response = app
        .clone()
        .oneshot(
            Request::post("/event")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_response.status(), StatusCode::CREATED);

    let get_response = app
        .oneshot(
            Request::get("/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    // ✅ קריאה נכונה של body ב-Hyper 1.x
    let body_bytes = get_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    let arr = json.as_array().expect("response must be array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["message"], "hi");
    assert_eq!(arr[0]["status"], "for_test");
}
