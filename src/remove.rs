use crate::{command::RemoveCommand, config::Config};
use color_eyre::{Result, eyre::eyre};
use std::path::PathBuf;

pub fn remove(RemoveCommand { name, force }: RemoveCommand) -> Result<()> {
    let mut config_path = std::env::current_exe()?;
    config_path.pop();
    config_path.push("gwt.toml");

    let config = std::fs::read_to_string(config_path)
        .ok()
        .and_then(|x| toml::from_str::<Config>(&x).ok())
        .unwrap_or_default();

    if name == config.default_branch {
        return Err(eyre!("Cannot delete develop worktree"));
    }

    let rev_parse = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if !rev_parse.status.success() {
        return Err(eyre!("git rev-parse failed"));
    }

    let mut directory = PathBuf::from(std::str::from_utf8(&rev_parse.stdout)?.trim_end());
    directory.pop();
    directory.push(name.replace('/', "__"));

    let mut command = std::process::Command::new("git");
    command.arg("worktree").arg("remove").arg(&directory);

    if force {
        command.arg("--force");
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(eyre!(
            "git worktree remove exited with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output = std::process::Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(&name)
        .output()?;

    if !output.status.success() {
        return Err(eyre!(
            "git branch -d exited with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
