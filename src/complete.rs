use crate::{
    branches::complete_branches,
    command::{CompleteCommand, CompleteSubommand},
};

pub fn complete(CompleteCommand { command }: CompleteCommand) {
    match command {
        CompleteSubommand::Branch { current } => {
            for branch in complete_branches(current.as_deref()) {
                println!("{branch}");
            }
        }
    }
}
