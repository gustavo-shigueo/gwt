use crate::{
    branches::{complete_branches, complete_worktree_branches},
    command::{CompleteCommand, CompleteSubommand},
};

pub fn complete(CompleteCommand { command }: CompleteCommand) {
    match command {
        CompleteSubommand::Switch { current } => {
            for branch in complete_branches(current.as_deref()) {
                println!("{branch}");
            }
        }

        CompleteSubommand::Remove { current } => {
            for branch in complete_worktree_branches(current.as_deref()) {
                println!("{branch}");
            }
        }
    }
}
