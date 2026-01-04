use crate::models::task::{ApiResponse, CreateTask, Task, UpdateTask};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::string::String;
use std::sync::{Arc, Mutex};
use validator::{Validate, ValidationErrors};

pub type SharedState = Arc<Mutex<Vec<Task>>>;

pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format_validation_errors(&errors),
            }),
        );
    }

    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;

    let new_task = Task {
        id,
        title: payload.title.clone(),
        done: false,
    };

    tasks.push(new_task.clone());

    (
        StatusCode::CREATED,
        Json(ApiResponse::Success(new_task.clone())),
    )
}

pub async fn list_tasks(State(state): State<SharedState>) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = state.lock().unwrap();

    (StatusCode::OK, Json(tasks.clone()))
}

pub async fn get_task(Path(id): Path<u32>, State(state): State<SharedState>) -> impl IntoResponse {
    let tasks = state.lock().unwrap();

    let task = match tasks.iter().find(|t| t.id == id) {
        Some(t) => t,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::Error {
                    message: "Task não encontrada".into(),
                }),
            );
        }
    };

    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}

pub async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format_validation_errors(&errors),
            }),
        );
    }

    let mut tasks = state.lock().unwrap();

    let task = match tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::Error {
                    message: "Task não encontrada".into(),
                }),
            );
        }
    };

    task.done = payload.done;
    task.title = payload.title.clone();

    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}

pub async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
) -> (StatusCode, Json<String>) {
    let mut tasks = state.lock().unwrap();

    let pos = tasks.iter().position(|t| t.id == id);

    match pos {
        Some(index) => {
            let task = tasks.remove(index);
            (
                StatusCode::OK,
                Json(format!("Task '{}' deletada!", task.title)),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(String::from("Task não encontrada")),
        ),
    }
}

// Helper da validação
fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            format!("{}: {}", field, messages.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}
