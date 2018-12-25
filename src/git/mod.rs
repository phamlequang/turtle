use super::cmd::Command;
use super::config::Repository;

pub fn clone(repository: &Repository) -> Command {
    let raw = format!("git clone {} {}", repository.remote, repository.local);
    return Command::new(&raw, "", true);
}
