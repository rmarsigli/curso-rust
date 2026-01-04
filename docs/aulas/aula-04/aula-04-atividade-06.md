# Rust - Aula 04 - Atividade 06

Arquivo: `src/main.rs`

```rust
mod models;
mod handlers;
mod routes;

use axum::{Router, routing::get};
use std::sync::{Arc, Mutex};
use handlers::tasks::SharedState;
use models::task::Task;

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
```

O que mudou:

* `mod models`, `mod handlers`, `mod routes` no topo
* `.nest("/tasks", routes::tasks::router())` usa função
* 30 linhas vs 300+
