use clap::Parser;
use color_eyre::Result;

mod command;
mod config;
mod init;
mod list;
mod remove;
mod switch;

use crate::command::Command;
use crate::init::init;
use crate::list::list;
use crate::remove::remove;
use crate::switch::switch;

fn main() -> Result<()> {
    let command = Command::parse();

    match command {
        Command::Switch(x) => switch(x),
        Command::Remove(x) => remove(x),
        Command::List => list(),
        Command::Init(x) => init(x),
    }
}
