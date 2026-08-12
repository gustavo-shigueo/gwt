use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub enum Command {
    /// Switch to a new or existing worktree
    Switch(SwitchCommand),

    /// Delete a worktree and its branch
    Remove(RemoveCommand),

    /// List current worktrees
    List,

    /// Enable gwt for your shell
    Init(InitCommand),
}

#[derive(Parser, Debug)]
pub struct SwitchCommand {
    /// Use this flag to create a new branch and switch into its new worktree
    #[arg(short)]
    pub create: bool,

    /// By default, whenever you use `-c`, the base branch will be "origin/develop".
    /// This flag makes it your current HEAD is the base for the new branch instead
    #[arg(long)]
    pub no_develop: bool,

    /// The name of the branch you wish to switch into
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct RemoveCommand {
    /// Name of the brach to be deleted
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct InitCommand {
    /// Which shell script should be generated
    pub shell: Shell,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab-case")]
pub enum Shell {
    Nu,
    Powershell,
}
