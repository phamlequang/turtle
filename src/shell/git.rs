use super::cmd::Command;
use crate::config::Repository;

pub fn clone(repository: &Repository) -> Command {
    let args = vec![
        String::from("clone"),
        repository.remote.clone(),
        repository.local.clone(),
    ];

    return Command::new("git", args, "", true);
}
