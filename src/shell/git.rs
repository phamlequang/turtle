use super::cmd::Command;
use crate::config::Repository;

pub fn clone(repository: &Repository) -> Command {
    let raw = format!("git clone {} {}", repository.remote, repository.local);
    return Command::new(&raw, "", true);
}
