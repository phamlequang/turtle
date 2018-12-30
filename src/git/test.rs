use super::*;
use crate::config::Repository;

#[test]
fn test_clone_repository() {
    let repository = Repository {
        name: "turtle".to_owned(),
        remote: "git@gitlab.com:phamlequang/turtle.git".to_owned(),
        local: "/Users/phamlequang/projects/turtle".to_owned(),
        branch: "master".to_owned(),
        services: None,
    };

    let command = clone_repository(&repository);
    let expect = "git clone -b master \
                  git@gitlab.com:phamlequang/turtle.git \
                  /Users/phamlequang/projects/turtle";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
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
