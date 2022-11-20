use anyhow::{anyhow, Result};
use std::fs::create_dir_all;
use std::path::Path;

use crate::{config, utils};

pub fn get(location: String, search: bool) -> Result<i32> {
    let cfg: config::Config = config::Config::load()?;

    if search {
        let mut possible_shortcuts: Vec<String> = vec![];

        for shortcut in &cfg.shortcuts {
            if shortcut.calls.iter().any(|c| c.starts_with(&location)) {
                for c in shortcut.calls.iter().cloned() {
                    if c.starts_with(&location) {
                        possible_shortcuts.push(c);
                        break;
                    }
                }
            }
        }

        println!("{}", possible_shortcuts.join(" "));
        return Ok(0);
    }

    for shortcut in cfg.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            let shortcut_location = utils::string::expand_path(shortcut.location.clone());

            if Path::new(&shortcut_location).exists() {
                println!("{}", shortcut_location);
                return Ok(0);
            }

            if !cfg.options.create_missing_directories {
                return Err(anyhow!(format!(
                    "Shortcut location does not exist {}. If you would like quicknav to automatically create missing directories for you, enable the option create_missing_directories in your config file.",
                    &shortcut_location,
                )));
            }

            create_dir_all(&shortcut_location)?;

            println!("{}", shortcut_location);
            return Ok(0);
        }
    }

    Err(anyhow!(
        "Navigation shortcut not found. Use quicknav list to view all your shortcuts."
    ))
}
