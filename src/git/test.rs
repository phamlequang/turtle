use super::*;
use crate::config::Repository;

#[test]
fn test_clone_repository() {
    let repository = Repository {
        name: String::from("turtle"),
        remote: String::from("git@gitlab.com:phamlequang/turtle.git"),
        local: String::from("/Users/phamlequang/projects/turtle"),
        branch: String::from("master"),
        services: None,
    };

    let command = clone_repository(&repository);
    let raw = format!(
        "git clone -b master {} {}",
        repository.remote, repository.local
    );
    let expect = Command::basic_show(&raw);
    assert_eq!(command, expect);
}

#[test]
fn test_pull_repository() {
    let repository = Repository {
        name: String::from("turtle"),
        remote: String::from("git@gitlab.com:phamlequang/turtle.git"),
        local: String::from("/Users/phamlequang/projects/turtle"),
        branch: String::from("master"),
        services: None,
    };

    let command = pull_repository(&repository);
    let expect = Command::new("git pull", &repository.local, true, false, false, None);

    assert_eq!(command, expect);
}

#[test]
fn test_current_branch() {
    let command = current_branch();

    assert_eq!(command.raw, "git branch");
    assert!(command.dir.is_empty());
    assert!(!command.show);
    assert!(command.pipe);
    assert!(command.then.is_some());

    let exec = command.then.unwrap();
    let (success, branch) = exec("master\n* feature\n");
    assert!(success);
    assert_eq!(branch, "* feature");
}
