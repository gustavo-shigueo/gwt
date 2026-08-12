use crate::command::RemoveCommand;
use color_eyre::{Result, eyre::eyre};
use std::path::PathBuf;

pub fn remove(RemoveCommand { name }: RemoveCommand) -> Result<()> {
    if name == "develop" {
        return Err(eyre!("Cannot delete develop worktree"));
    }

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

    let output = std::process::Command::new("git")
        .arg("worktree")
        .arg("remove")
        .arg(&directory)
        .output()?;

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

    println!("{}", repo_root.display());

    Ok(())
}
