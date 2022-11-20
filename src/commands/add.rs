use anyhow::{anyhow, Result};
use colored::*;

use crate::{config, utils};

pub fn add(
    call: String,
    location: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<i32> {
    let mut cfg: config::Config = config::Config::load()?;

    for shortcut_conf in &mut cfg.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &call) {
            return Err(anyhow!(format!(
                "Shortcut with call {} already exists. Consider using {}.",
                &call,
                format!("quicknav edit {}", &call).yellow()
            )));
        }
    }

    let mut shortcut_name: String = call.to_string();
    let mut shortcut_description: String = call.to_string();
    let shortcut_location = utils::string::normalize_path(&location)?;

    if let Some(name) = name {
        shortcut_name = name;
    }

    if let Some(description) = description {
        shortcut_description = description;
    }

    let new_shortcut = config::Shortcut {
        name: shortcut_name,
        description: shortcut_description,
        location: shortcut_location,
        calls: vec![call.to_string()],
    };

    cfg.shortcuts.push(new_shortcut);
    cfg.update()?;
    println!("{} {}", "New shortcut added:".green(), &call);

    Ok(0)
}

pub fn add_call(shortcut: String, call: String) -> Result<i32> {
    let mut cfg: config::Config = config::Config::load()?;
    let mut found_shortcut = false;
    let mut shortcut_index: usize = 0;

    for (i, shortcut_conf) in &mut cfg.shortcuts.iter().enumerate() {
        if shortcut_conf.name.to_lowercase() == shortcut.to_lowercase() {
            found_shortcut = true;
            shortcut_index = i;
        }
    }

    if !found_shortcut {
        return Err(anyhow!(format!(
            "Shortcut with name {} was not found.",
            shortcut,
        )));
    }

    for shortcut_conf in &mut cfg.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &call) {
            return Err(anyhow!(format!(
                "Call {} already exists on the shortcut named {}.",
                &call, shortcut_conf.name
            )));
        }
    }

    cfg.shortcuts[shortcut_index].calls.push(call.to_owned());
    cfg.update()?;
    println!("{} {}", "New call added:".green(), &call);

    Ok(0)
}
