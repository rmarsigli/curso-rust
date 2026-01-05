mod error;
mod handlers;
mod models;
mod routes;
mod validators;

use axum::{Router, routing::get};
use handlers::tasks::SharedState;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
        .nest("/tasks", routes::tasks::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();

    println!("🚀 Server rodando em http://127.0.0.1:3003");

    axum::serve(listener, app).await.unwrap();
}
