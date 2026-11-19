use std::path::PathBuf;

use color_eyre::{Result, eyre::eyre};

use crate::config::Config;

pub fn setup() -> Result<()> {
    let mut config_path = std::env::current_exe()?;
    config_path.pop();
    config_path.push("gwt.toml");

    let config = std::fs::read_to_string(config_path)
        .ok()
        .and_then(|x| toml::from_str::<Config>(&x).ok())
        .unwrap_or_default();

    let git_branch = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;

    if !git_branch.status.success() {
        return Err(eyre!("git branch failed"));
    }

    let current_branch = std::str::from_utf8(&git_branch.stdout)?.trim();
    if current_branch != config.default_branch {
        return Err(eyre!(
            "You must be in your default branch to run the setup command."
        ));
    }

    let rev_parse = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if !rev_parse.status.success() {
        return Err(eyre!("git rev-parse failed"));
    }

    let current_directory = PathBuf::from(std::str::from_utf8(&rev_parse.stdout)?.trim_end());

    let target_directory_name = current_branch.replace('/', "__");
    let target_directory = current_directory.join(&target_directory_name);

    std::fs::create_dir(&target_directory)?;

    for entry in std::fs::read_dir(&current_directory)? {
        let entry = entry?;
        if entry.file_name().to_str().unwrap_or_default() == target_directory_name {
            continue;
        }

        std::fs::rename(entry.path(), target_directory.join(entry.file_name()))?;
    }

    Ok(())
}
