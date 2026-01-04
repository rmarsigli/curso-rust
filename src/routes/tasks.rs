use axum::{Router, routing::{get, post, patch, delete}};
use crate::handlers::tasks::*;
use crate::handlers::tasks::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/tasks/{id}", delete(delete_task))
        .route("/tasks/{id}", patch(update_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks", post(create_task))
        .route("/tasks", get(list_tasks))
}