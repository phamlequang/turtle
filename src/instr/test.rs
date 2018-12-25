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

    let expect = Command::echo("goodbye!");
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

    let repository = config.search_repository("flowers").unwrap();
    let expect = git::clone_repository(repository);
    assert_eq!(&commands[0], &expect);

    let expect = Command::echo("--> unknown repository [ tree ]");
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

    let expect = docker::create_machine(&config.docker_machine);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_docker_machine_remove() {
    let config = Config::default();
    let args = vec!["rm".to_owned()];

    let instruction = Instruction::docker_machine(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = docker::machine_command("rm", &config.docker_machine);
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

    let expect = docker::update_certificates(&config.docker_machine);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_docker_machine_start() {
    let config = Config::default();
    let machine = &config.docker_machine;

    let args = vec!["start".to_owned()];
    let instruction = Instruction::docker_machine(args, &config);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 2);

    let expect = docker::machine_command("start", machine);
    assert_eq!(&commands[0], &expect);

    let expect = docker::load_environments(&machine);
    assert_eq!(&commands[1], &expect);
}
