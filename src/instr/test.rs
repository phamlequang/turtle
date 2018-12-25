use super::*;

#[test]
fn test_do_nothing() {
    let instruction = Instruction::do_nothing();
    assert!(instruction.commands.is_empty());
    assert!(!instruction.should_terminate)
}

#[test]
fn test_terminate() {
    let instruction = Instruction::terminate();
    assert!(instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new("echo \"goodbye!\"", "", false);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_other() {
    let instruction = Instruction::other("ls -la");
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new("ls -la", "", false);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_clone_repository() {
    let config = Config::default();
    let args = vec!["flowers".to_owned(), "tree".to_owned()];

    let instruction = Instruction::clone_repositories(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 2);

    let expect = Command::new(
        "git clone -b master \
         git@gitlab.com:phamlequang/flowers.git \
         /Users/phamlequang/projects/flowers",
        "",
        true,
    );
    assert_eq!(&commands[0], &expect);

    let expect = Command::new("echo \"--> unknown repository [ tree ]\"", "", false);
    assert_eq!(&commands[1], &expect);
}

#[test]
fn test_change_directory() {
    let args = vec!["..".to_owned()];
    let instruction = Instruction::change_directory(args);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new("", "..", false);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_docker_machine_create() {
    let config = Config::default();
    let args = vec!["create".to_owned()];

    let instruction = Instruction::docker_machine(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new(
        "docker-machine create \
         --driver virtualbox \
         --virtualbox-host-dns-resolver \
         --virtualbox-cpu-count 2 \
         --virtualbox-disk-size 16384 \
         --virtualbox-memory 4096 \
         turtle",
        "",
        true,
    );
    assert_eq!(&commands[1], &expect);
}

#[test]
fn test_docker_machine_remove() {
    let config = Config::default();
    let args = vec!["rm".to_owned()];

    let instruction = Instruction::docker_machine(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new("docker-machine rm turtle", "", true);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_docker_machine_update_certificates() {
    let config = Config::default();
    let args = vec!["upcerts".to_owned()];

    let instruction = Instruction::docker_machine(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::new(
        "docker-machine regenerate-certs --force --client-certs turtle",
        "",
        true,
    );
    assert_eq!(&commands[0], &expect);
}
