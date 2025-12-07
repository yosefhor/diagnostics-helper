mod db;
mod handlers;
mod models;
mod router;

use crate::db::init_db;
use crate::router::create_router;
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = init_db();
    let app = create_router(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("starting server!!");
    println!("Server running at http://127.0.0.1:3000");
    serve(listener, app).await.unwrap();
}
