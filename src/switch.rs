use crate::{command::SwitchCommand, config::Config};
use color_eyre::{Result, eyre::eyre};
use std::path::PathBuf;

pub fn switch(
    SwitchCommand {
        name,
        create,
        no_commit_ish,
    }: SwitchCommand,
) -> Result<()> {
    let mut config_path = std::env::current_exe()?;
    config_path.pop();
    config_path.push("gwt.toml");

    let config = std::fs::read_to_string(config_path)
        .ok()
        .and_then(|x| toml::from_str::<Config>(&x).ok())
        .unwrap_or_default();

    let rev_parse = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if !rev_parse.status.success() {
        return Err(eyre!("git rev-parse failed"));
    }

    let repo_root = PathBuf::from(std::str::from_utf8(&rev_parse.stdout)?.trim_end());
    let mut directory = repo_root.clone();
    directory.pop();
    directory.push(name.replace('/', "__"));

    if create {
        let status = std::process::Command::new("git")
            .arg("fetch")
            .arg(&config.remote)
            .status()?;
        if !status.success() {
            return Err(eyre!("git fetch failed"));
        }
    }

    if directory.is_dir() {
        if create {
            return Err(eyre!("A directory named after this branch already exists"));
        }

        println!("{}", directory.display());
        return Ok(());
    }

    let mut command = std::process::Command::new("git");
    command.arg("worktree").arg("add").arg(&directory);
    if create {
        command.arg("-b");
    }

    command.arg(&name);

    if create && !no_commit_ish {
        let commit_ish = format!("{}/{}", config.remote, config.default_branch);
        command.arg(commit_ish);
    }

    let output = command.output()?;
    if !output.status.success() {
        return Err(eyre!(
            "git worktree add exited with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let original_env = repo_root.join(".env");
    if original_env.is_file() {
        let new_env = directory.join(".env");
        std::fs::copy(&original_env, &new_env)?;
    }

    let original_node_modules = repo_root.join("node_modules");
    if original_node_modules.is_dir() {
        let new_node_modules = directory.join("node_modules");

        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_dir(&original_node_modules, &new_node_modules)?;

        #[cfg(target_family = "unix")]
        std::os::unix::fs::symlink(&original_node_modules, &new_node_modules)?;
    }

    println!("{}", directory.display());

    Ok(())
}
