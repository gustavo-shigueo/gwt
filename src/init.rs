use crate::{
    command::{InitCommand, Shell},
    config::Config,
};
use color_eyre::{Result, eyre::eyre};

pub fn init(
    InitCommand {
        shell,
        default_branch,
        remote,
    }: InitCommand,
) -> Result<()> {
    if default_branch.contains('\'') || default_branch.contains("__") {
        return Err(eyre!(
            r#"gwt cannot handle branch names that include "'" or "__""#
        ));
    }

    let function = match shell {
        Shell::Nu => init_nu(&default_branch),
        Shell::Powershell => init_powershell(&default_branch),
        Shell::Bash => init_bash(&default_branch),
    };

    let mut config_path = std::env::current_exe()?;
    config_path.pop();
    config_path.push("gwt.toml");

    let config = Config {
        default_branch,
        remote,
    };

    let buffer = toml::to_string(&config)?;
    std::fs::write(config_path, buffer)?;

    println!("{}", function.trim());
    Ok(())
}

fn init_nu(default_branch: &str) -> String {
    let default_branch = default_branch.replace('/', "__");
    include_str!("../assets/init_nu.nu").replace("_GWT_DEFAULT_BRANCH_", &default_branch)
}

fn init_powershell(default_branch: &str) -> String {
    let default_branch = default_branch.replace('/', "__");
    include_str!("../assets/init_powershell.ps1").replace("_GWT_DEFAULT_BRANCH_", &default_branch)
}

fn init_bash(default_branch: &str) -> String {
    let default_branch = default_branch.replace('/', "__");
    include_str!("../assets/init_bash.sh").replace("_GWT_DEFAULT_BRANCH_", &default_branch)
}
