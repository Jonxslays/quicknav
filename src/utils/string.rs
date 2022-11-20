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
    let home_dir: String;

    if let Some(suffix) = path.strip_prefix('.') {
        let mut prefix = env::current_dir()?.display().to_string();
        if !suffix.is_empty() {
            prefix.push_str(suffix);
        }

        result = prefix;
    } else if path.starts_with('~') && env::consts::OS == "windows" {
        // Manually replace the ~ with the windows home dir
        result = fs::canonicalize(env::var("USERPROFILE")? + &path[1..])?
            .display()
            .to_string();
    } else {
        result = fs::canonicalize(path)?.display().to_string();
    }

    if env::consts::OS == "windows" {
        home_dir = env::var("USERPROFILE")?;
        if let Some(res) = result.strip_prefix(r#"\\?\"#) {
            // This prefix is prepended by canonicalize on a windows path
            // for some reason
            result = res.to_string();
        }
    } else {
        home_dir = env::var("HOME")?;
    }

    if let Some(res) = result.strip_prefix(&home_dir) {
        result = "~".to_string() + res;
    }

    Ok(result)
}

pub fn expand_path(path: String) -> String {
    let mut result = String::new();

    if path.starts_with('~') {
        if env::var("HOME").is_ok() {
            // Unix
            result.push_str(&env::var("HOME").unwrap());
        } else if env::var("USERPROFILE").is_ok() {
            // Windows
            result.push_str(&env::var("USERPROFILE").unwrap());
        }
    }

    result.push_str(path.strip_prefix('~').unwrap_or(&path));
    result
}
