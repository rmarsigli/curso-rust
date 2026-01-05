use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Task não encontrada")]
    NotFound,

    #[error("Erro de validação: {0}")]
    ValidationError(String),

    #[error("Erro ao travar mutex")]
    MutexError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::MutexError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}
