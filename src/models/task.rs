use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(
        min = 4,
        max = 100,
        message = "Título deve ter entre 4 e 100 caracteres",
    ))]
    pub title: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(length(
        min = 4,
        max = 100,
        message = "Título deve ter entre 4 e 100 caracteres",
    ))]
    pub title: String,
    // Aqui não precisa de validação, bool existe sempre bool
    pub done: bool,
}
