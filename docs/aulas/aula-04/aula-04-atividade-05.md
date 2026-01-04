# Rust - Aula 04 - Atividade 05

# Atividade 05 - Mover Routes

Arquivo: `src/routes/tasks.rs`

```rust
use axum::{Router, routing::{get, post, patch, delete}};
use crate::handlers::tasks::*;
use crate::handlers::tasks::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_tasks).post(create_task))
        .route("/{id}", get(get_task).patch(update_task).delete(delete_task))
}
```

O que **mudou**:

* Router agora é **função** que retorna Router
* `Router<SharedState>` = generic type

Arquivo: `src/routes/mod.rs`

```rust
pub mod tasks;
```
