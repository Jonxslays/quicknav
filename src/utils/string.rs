use anyhow::{anyhow, Result};

pub fn to_bool(string: &str) -> Result<bool> {
    match string.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => {
            return Err(anyhow!(format!(
                "The argument {} is not of type bool. The value must be true or false.",
                string
            )))
        }
    }
}
