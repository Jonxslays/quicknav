use anyhow::{anyhow, Result};
use std::{env, fs};

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

pub fn normalize_path(path: &str) -> String {
    let mut result: String;

    if path.starts_with(".") {
        let mut prefix = env::current_dir().unwrap().display().to_string();

        if path == "." {
            result = prefix;
        } else {
            prefix.push_str(&path[1..]);
            result = prefix;
        }
    } else {
        result = fs::canonicalize(path).unwrap().display().to_string();
    }

    if env::consts::OS != "windows" {
        let home = env::var("HOME").unwrap();
        if result.starts_with(&home) {
            // Only auto-shorten the path to ~ when not on windows
            // Windows drive letters cause issues
            result = "~".to_string() + result.strip_prefix(&home).unwrap();
        }
    }

    result
}

pub fn expand_path(path: String) -> String {
    let mut result = String::new();

    if path.starts_with("~") {
        if env::var("HOME").is_ok() {
            result.push_str(&env::var("HOME").unwrap());
        } else if env::var("HOMEPATH").is_ok() {
            result.push_str(&env::var("HOMEPATH").unwrap());
        }
    }

    result.push_str(path.strip_prefix("~").unwrap_or(&path));
    result
}
