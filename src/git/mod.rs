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

pub fn current_branch() -> Command {
    let raw = "git branch";

    let exec = |stdout: &str| -> (bool, String) {
        for branch in stdout.lines() {
            if branch.starts_with("*") {
                return (true, branch.to_owned());
            }
        }
        return (true, String::new());
    };

    return Command::new(raw, "", false, true, true, Some(Box::new(exec)));
}
