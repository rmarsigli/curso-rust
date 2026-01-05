use crate::validators::text::validate_title;
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
    #[validate(custom(function = "validate_title"))]
    pub title: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    pub done: bool,
}
