use clap::{Command, CommandFactory, Parser};
use clap_complete::CompleteEnv;
use color_eyre::Result;

mod branches;
mod command;
mod complete;
mod config;
mod init;
mod list;
mod remove;
mod switch;
mod sync;

use crate::command::Cli;
use crate::complete::complete;
use crate::init::init;
use crate::list::list;
use crate::remove::remove;
use crate::switch::switch;
use crate::sync::sync;

fn command() -> Command {
    let mut cmd = Cli::command();
    cmd.set_bin_name("gwt");
    cmd
}

fn main() -> Result<()> {
    CompleteEnv::with_factory(command).complete();

    let command = Cli::parse();

    match command {
        Cli::Switch(x) => switch(x),
        Cli::Sync => sync(),
        Cli::Remove(x) => remove(x),
        Cli::List => list(),
        Cli::Init(x) => init(x),
        Cli::Complete(x) => {
            complete(x);
            Ok(())
        }
    }
}
