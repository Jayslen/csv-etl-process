pub fn validate_columns(cols: &[&str], expected: usize) -> Result<(), String> {
    if cols.len() != expected {
        return Err(format!(
            "Invalid column count: expected {}, got {}",
            expected,
            cols.len()
        ));
    }
    Ok(())
}

pub fn validate_not_empty(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("Field '{}' is empty", field));
    }
    Ok(())
}
