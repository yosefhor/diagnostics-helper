mod models;
mod db;
mod handlers;
mod router;

use axum::serve;
use tokio::net::TcpListener;
use crate::db::init_db;
use crate::router::create_router;

#[tokio::main]
async fn main() {
    let db = init_db();
    let app = create_router(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}
