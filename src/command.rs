use clap::{Parser, ValueEnum};
use clap_complete::ArgValueCompleter;

use crate::branches::complete_branch_candidates;

#[derive(Parser, Debug)]
pub enum Cli {
    /// Switch to a new or existing worktree
    Switch(SwitchCommand),

    /// Delete a worktree and its branch
    Remove(RemoveCommand),

    /// List current worktrees
    List,

    /// Enable gwt for your shell
    Init(InitCommand),

    Complete(CompleteCommand),
}

#[derive(Parser, Debug)]
pub struct SwitchCommand {
    /// Use this flag to create a new branch and switch into its new worktree
    #[arg(short)]
    pub create: bool,

    /// By default, whenever you use `-c`, the base branch will be "`remote`/`default_branch`".
    /// This flag makes it your current HEAD is the base for the new branch instead
    #[arg(long)]
    pub no_commit_ish: bool,

    /// The name of the branch you wish to switch into
    #[arg(add = ArgValueCompleter::new(complete_branch_candidates))]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct RemoveCommand {
    /// Name of the brach to be deleted
    #[arg(add = ArgValueCompleter::new(complete_branch_candidates))]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct InitCommand {
    /// Name of the repo's remote
    #[arg(long, default_value = "origin")]
    pub remote: String,

    /// Default branch of the repo. Will be used as the `commit_ish`
    /// for `git worktree`
    #[arg(long, default_value = "develop")]
    pub default_branch: String,

    /// Which shell script should be generated
    pub shell: Shell,
}

#[derive(Parser, Debug)]
pub struct CompleteCommand {
    #[command(subcommand)]
    pub command: CompleteSubommand,
}

#[derive(Parser, Debug)]
pub enum CompleteSubommand {
    Branch { current: Option<String> },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab-case")]
pub enum Shell {
    Nu,
    Powershell,
}
