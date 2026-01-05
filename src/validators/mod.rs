pub mod text;

use validator::ValidationError;

pub fn length_between(
    value: &str,
    min: usize,
    max: usize,
    field_name: &str,
) -> Result<(), ValidationError> {
    let len = value.chars().count();

    if len < min || len > max {
        let mut error = ValidationError::new("length_between");
        error.message =
            Some(format!("{} deve ter entre {} e {} caracteres", field_name, min, max).into());
        error.add_param("min".into(), &min);
        error.add_param("max".into(), &max);
        error.add_param("actual".into(), &len);

        return Err(error);
    }

    Ok(())
}

pub fn not_empty(value: &str, field_name: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        let mut error = ValidationError::new("not_empty");
        error.message = Some(format!("{} não pode estar vazio", field_name).into());
        return Err(error);
    }
    Ok(())
}
