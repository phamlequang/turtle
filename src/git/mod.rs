#[cfg(test)]
mod test;

use super::cmd::Command;
use super::config::Repository;

pub fn clone_repository(repository: &Repository) -> Command {
    let raw = format!(
        "git clone -b {} {} {}",
        repository.branch, repository.remote, repository.local
    );
    return Command::basic_show(&raw);
}
