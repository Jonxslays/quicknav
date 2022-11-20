use anyhow::Result;
use quicknav::Quicknav;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::quicknav;
use crate::utils::CompletionWriter;

fn get_profile(profile: &str, command: &str) -> Result<String> {
    if profile == "default" {
        let shell_profile = include_str!("../../shell/default.txt");

        if !command.is_empty() {
            let new_command = format!("function {}", command);
            return Ok(shell_profile.replace("function nav", &new_command));
        }

        return Ok(shell_profile.to_string());
    } else if profile == "fish" {
        let shell_profile = include_str!("../../shell/fish.txt");

        if !command.is_empty() {
            let new_command = format!("function {}", command);
            return Ok(shell_profile.replace("function nav", &new_command));
        }

        return Ok(shell_profile.to_string());
    }

    Ok("".to_string())
}

fn gen_completions(shell: String) -> Result<i32> {
    let mut shell_profile = Shell::Bash;

    if shell == "bash" {
        shell_profile = Shell::Bash;
    } else if shell == "zsh" {
        shell_profile = Shell::Bash;

        let mut writer = CompletionWriter::new();
        Quicknav::clap().gen_completions_to("quicknav", shell_profile, &mut writer);

        let completions = writer.clone_content();
        drop(writer);

        println!(
            "autoload bashcompinit\nbashcompinit\n\n{}",
            completions.replace(
                "complete -F _quicknav -o bashdefault -o default quicknav",
                "$(autoload | grep -q bashcompinit) && \
                 complete -F _quicknav -o bashdefault -o default quicknav"
            )
        );

        return Ok(0);
    } else if shell == "fish" {
        shell_profile = Shell::Fish;
    }

    Quicknav::clap().gen_completions_to("quicknav", shell_profile, &mut std::io::stdout());

    Ok(0)
}

fn get_nav_completions(shell: String) -> Result<String> {
    if shell == "bash" {
        let completions_profile = include_str!("../../shell/completions/default.txt");
        return Ok(completions_profile.to_string());
    } else if shell == "zsh" {
        let completions_profile = include_str!("../../shell/completions/default.txt");
        return Ok(completions_profile.to_string().replace(
            "complete -F _nav -o bashdefault -o default nav",
            "$(autoload | grep -q bashcompinit) && \
                 complete -F _nav -o bashdefault -o default nav",
        ));
    }

    Ok("".to_string())
}

pub fn init(shell: String, command: Option<String>) -> Result<i32> {
    let supported_shells = vec!["bash", "zsh", "fish"];

    if supported_shells.iter().any(|&s| s == shell) {
        gen_completions(shell.to_owned())?;
    } else {
        println!("echo -e \"\\033[0;31mError:\\033[0m Failed to load shell profile. Invalid or unsupported shell provided.\"");
        return Ok(1);
    }

    println!("{}", get_nav_completions(shell.to_owned())?);

    let mut profile: &str = "default";

    if shell == "bash" || shell == "zsh" {
        profile = "default"
    } else if shell == "fish" {
        profile = "fish"
    }

    if let Some(cmd) = command {
        println!("{}", get_profile(profile, &cmd)?);
    } else {
        println!("{}", get_profile(profile, "")?);
    }

    Ok(0)
}
