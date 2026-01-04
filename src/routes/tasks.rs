use axum::{Router, routing::{get, post, patch, delete}};
use crate::handlers::tasks::*;
use crate::handlers::tasks::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/{id}", delete(delete_task))
        .route("/{id}", patch(update_task))
        .route("/{id}", get(get_task))
        .route("/", post(create_task))
        .route("/", get(list_tasks))
}