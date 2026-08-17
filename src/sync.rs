use crate::config::Config;

use color_eyre::{Result, eyre::eyre};

pub fn sync() -> Result<()> {
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

    println!("Getting references from remote repository");

    let fetch_command = std::process::Command::new("git")
        .arg("fetch")
        .arg(&config.remote)
        .status()?;

    if !fetch_command.success() {
        return Err(eyre!("git fetch failed"));
    }


    let merge_command = std::process::Command::new("git")
        .arg("merge")
        .arg(format!("{}/{}", config.remote, config.default_branch))
        .status()?;

    if !merge_command.success() {
        let merge_has_conflict = std::process::Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--diff-filter=U")
            .output()?;

        if std::str::from_utf8(&merge_has_conflict.stdout)?
            .trim()
            .is_empty()
        {
            return Err(eyre!("git merge failed"));
        }

        return Ok(());
    }


    Ok(())
}
