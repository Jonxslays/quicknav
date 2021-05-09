use anyhow::Result;
use colored::*;
use prettytable::format;
use prettytable::Table;

use crate::config;

pub fn list(shortcut: Option<String>) -> Result<i32> {
    if let Some(call) = shortcut {
        let config: config::Config = config::load_config()?;

        let mut shortcut_list = Table::new();
        shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        shortcut_list.set_titles(row!["Shortcuts", "Shortcut Name", "Shortcut Location"]);

        for shortcut_conf in config.shortcuts {
            if shortcut_conf.calls.iter().any(|c| c == &call) {
                let calls: String = shortcut_conf.calls.join(", ");
                shortcut_list.add_row(row![calls, shortcut_conf.name, shortcut_conf.location]);
                shortcut_list.printstd();
                return Ok(0);
            }
        }

        println!(
            "{} {}",
            "Error: Could not find shortcut with a call of".red(),
            call.red()
        );
        Ok(1)
    } else {
        let config: config::Config = config::load_config()?;

        let mut shortcut_list = Table::new();
        shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        shortcut_list.set_titles(row!["Shortcuts", "Shortcut Name", "Shortcut Location"]);

        for shortcut_conf in config.shortcuts {
            let calls: String = shortcut_conf.calls.join(", ");
            shortcut_list.add_row(row![calls, shortcut_conf.name, shortcut_conf.location]);
        }

        shortcut_list.printstd();
        Ok(0)
    }
}
