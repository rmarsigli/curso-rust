use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(Task),
    Error { message: String },
}

#[derive(Clone, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: String,
    pub done: bool,
}