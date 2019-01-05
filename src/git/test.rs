use super::*;
use crate::config::Repository;

#[test]
fn test_clone_repository() {
    let repository = Repository {
        name: String::from("turtle"),
        remote: String::from("git@gitlab.com:phamlequang/turtle.git"),
        local: String::from("/Users/phamlequang/projects/turtle"),
    };

    let command = clone_repository(&repository);
    let raw = format!("git clone {} {}", repository.remote, repository.local);

    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, "");
    assert!(command.show);
    assert!(!command.silent);
    assert!(!command.pipe);
    assert!(command.then.is_none());
}

#[test]
fn test_pull_repository() {
    let repo_dir = "/Users/phamlequang/projects/turtle";
    let raw = "git pull origin $(git branch | grep -m1 \\* | \
               grep -v \"master\" | grep -v \"HEAD detached\" | cut -c 3-)";

    let command = pull_repository(repo_dir);
    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, repo_dir);
    assert!(command.show);
    assert!(!command.silent);
    assert!(!command.pipe);
    assert!(command.then.is_none());
}

#[test]
fn test_push_repository() {
    let repo_dir = "/Users/phamlequang/projects/turtle";
    let raw = "git push origin $(git branch | grep -m1 \\* | \
               grep -v \"master\" | grep -v \"HEAD detached\" | cut -c 3-)";

    let command = push_repository(repo_dir);
    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, repo_dir);
    assert!(command.show);
    assert!(!command.silent);
    assert!(!command.pipe);
    assert!(command.then.is_none());
}

#[test]
fn test_current_branch() {
    let command = current_branch();

    assert_eq!(command.raw, "git branch");
    assert!(command.dir.is_empty());
    assert!(!command.show);
    assert!(command.silent);
    assert!(command.pipe);
    assert!(command.then.is_some());

    let exec = command.then.unwrap();
    let (success, branch) = exec("master\n* feature\n");
    assert!(success);
    assert_eq!(branch, "* feature");
}
