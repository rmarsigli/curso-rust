use crate::error::AppError;
use crate::models::task::{CreateTask, Task, UpdateTask};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::string::String;
use std::sync::{Arc, Mutex};
use validator::{Validate, ValidationErrors};

pub type SharedState = Arc<Mutex<Vec<Task>>>;

pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(format_validation_errors(&errors)));
    }

    let mut tasks = state.lock().map_err(|_| AppError::MutexError)?;
    let id = tasks.len() as u32 + 1;

    let new_task = Task {
        id,
        title: payload.title.clone(),
        done: false,
    };

    tasks.push(new_task.clone());

    Ok((StatusCode::CREATED, Json(new_task)))
}

pub async fn list_tasks(State(state): State<SharedState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = state.lock().map_err(|_| AppError::MutexError)?;

    Ok(Json(tasks.clone()))
}

pub async fn get_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
) -> Result<Json<Task>, AppError> {
    let tasks = state.lock().map_err(|_| AppError::MutexError)?;
    let task = tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;

    Ok(Json(task.clone()))
}

pub async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(format_validation_errors(&errors)));
    }

    let mut tasks = state.lock().map_err(|_| AppError::MutexError)?;

    let task = match tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        None => {
            return Err(AppError::NotFound);
        }
    };

    task.done = payload.done;
    task.title = payload.title.clone();

    Ok(Json(task.clone()))
}

pub async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
) -> Result<Json<String>, AppError> {
    let mut tasks = state.lock().map_err(|_| AppError::MutexError)?;
    let pos = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(AppError::NotFound)?;
    let task = tasks.remove(pos);

    Ok(Json(format!("Task '{}' deletada!", task.title)))
}

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
