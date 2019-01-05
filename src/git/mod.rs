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

pub fn pull_repository(repo_dir: &str) -> Command {
    return do_on_current_branch(repo_dir, "pull");
}

pub fn push_repository(repo_dir: &str) -> Command {
    return do_on_current_branch(repo_dir, "push");
}

pub fn do_on_current_branch(repo_dir: &str, action: &str) -> Command {
    let branch = "git branch | grep -m1 \\* | grep -v \"master\" | \
                  grep -v \"HEAD detached\" | cut -c 3-";
    let raw = format!("git {} origin $({})", action, branch);
    return Command::new(&raw, repo_dir, true, false, false, None);
}

pub fn current_branch() -> Command {
    let raw = "git branch";

    let exec = |stdout: &str| -> (bool, String) {
        for branch in stdout.lines() {
            if branch.starts_with("*") {
                return (true, String::from(branch));
            }
        }
        return (true, String::new());
    };

    return Command::new(raw, "", false, true, true, Some(Box::new(exec)));
}
