use super::cmd::Command;
use crate::config::*;

pub fn clone(repository: &Repository) -> Command {
    let program = String::from("git");

    let args = vec![
        String::from("clone"),
        repository.remote.clone(),
        repository.local.clone(),
    ];

    return Command::new(program, args, true);
}
