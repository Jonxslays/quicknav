use anyhow::{anyhow, Result};
use std::{env, fs};

pub fn to_bool(string: &str) -> Result<bool> {
    match string.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(anyhow!(format!(
            "The argument {} is not of type bool. The value must be true or false.",
            string
        ))),
    }
}

pub fn normalize_path(path: &str) -> Result<String> {
    let mut result: String;

    if let Some(suffix) = path.strip_prefix('.') {
        let mut prefix = env::current_dir()?.display().to_string();

        if !suffix.is_empty() {
            prefix.push_str(suffix);
        }

        result = prefix;
    } else {
        result = fs::canonicalize(path)?.display().to_string();
    }

    if env::consts::OS == "windows" {
        if let Some(res) = result.strip_prefix(r#"\\?\"#) {
            // This prefix is appended by canonicalize on a windows path
            // for some reason
            result = res.to_string();
        }
    } else if let Some(res) = result.strip_prefix(&env::var("HOME")?) {
        // Only auto-shorten the path to ~ when not on windows due to
        // drive letters causing issues
        result = "~".to_string() + res;
    }

    Ok(result)
}

pub fn expand_path(path: String) -> String {
    let mut result = String::new();

    if path.starts_with('~') {
        if env::var("HOME").is_ok() {
            result.push_str(&env::var("HOME").unwrap());
        } else if env::var("HOMEPATH").is_ok() {
            // Windows home variable - We should be able to expand this
            // with no issues like in the normalize function above
            result.push_str(&env::var("HOMEPATH").unwrap());
        }
    }

    result.push_str(path.strip_prefix('~').unwrap_or(&path));
    result
}
