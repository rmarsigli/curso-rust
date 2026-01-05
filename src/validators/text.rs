use crate::validators::{length_between, not_empty};
use validator::ValidationError;

pub fn validate_title(value: &str) -> Result<(), ValidationError> {
    length_between(value, 4, 100, "Título")?;
    not_empty(value, "Texto")
}
