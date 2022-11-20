use anyhow::{anyhow, Result};
use colored::*;

use crate::{config, utils};

pub fn edit(
    shortcut: String,
    location: Option<String>,
    name: Option<String>,
    description: Option<String>,
) -> Result<i32> {
    let mut cfg = config::Config::load()?;
    let mut res = "Shortcut edited: ".green().to_string();
    let mut valid_shortcut = false;

    for shortcut_conf in &mut cfg.shortcuts {
        if shortcut_conf.name.to_lowercase() == shortcut.to_lowercase() {
            valid_shortcut = true;

            if let (None, None, None) = (&location, &name, &description) {
                return Err(anyhow!("No data was provided to edit {}.", shortcut));
            }

            match &name {
                Some(n) => {
                    shortcut_conf.name = n.to_owned();
                    res.push_str(n);
                }
                _ => res.push_str(&shortcut_conf.name),
            }

            if let Some(location) = &location {
                shortcut_conf.location = utils::string::normalize_path(location);
                res.push_str(&format!("\nLocation: {}", &shortcut_conf.location));
            }

            if let Some(description) = &description {
                shortcut_conf.description = description.to_owned();
                res.push_str(&format!("\nDescription: {}", &description));
            }
        }
    }

    if !valid_shortcut {
        return Err(anyhow!(format!(
            "Shortcut with name {} was not found.",
            shortcut,
        )));
    }

    cfg.update()?;
    println!("{}", res);

    Ok(0)
}
