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

    let current_worktree = PathBuf::from(std::str::from_utf8(&rev_parse.stdout)?.trim_end());
    let mut target_worktree = current_worktree.clone();
    target_worktree.pop();
    target_worktree.push(name.replace('/', "__"));

    if create && !no_commit_ish {
        let status = std::process::Command::new("git")
            .arg("fetch")
            .arg(&config.remote)
            .status()?;
        if !status.success() {
            return Err(eyre!("git fetch failed"));
        }
    }

    if target_worktree.is_dir() {
        if create {
            return Err(eyre!("A directory named after this branch already exists"));
        }

        println!("{}", target_worktree.display());
        return Ok(());
    }

    let mut command = std::process::Command::new("git");
    command.arg("worktree").arg("add").arg(&target_worktree);
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

    let mut main_worktree = current_worktree;
    main_worktree.pop();
    main_worktree.push(config.default_branch.replace('/', "__"));

    let original_env = main_worktree.join(".env");
    if original_env.is_file() {
        let new_env = target_worktree.join(".env");
        std::fs::copy(&original_env, &new_env)?;
    }

    let original_node_modules = main_worktree.join("node_modules");
    if original_node_modules.is_dir() {
        let new_node_modules = target_worktree.join("node_modules");

        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_dir(&original_node_modules, &new_node_modules)?;

        #[cfg(target_family = "unix")]
        std::os::unix::fs::symlink(&original_node_modules, &new_node_modules)?;
    }

    println!("{}", target_worktree.display());

    Ok(())
}
