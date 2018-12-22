use super::*;
use crate::config::Config;

fn empty_config() -> Config {
    return Config {
        docker_machine: None,
        dependencies: None,
        repositories: None,
        groups: None,
    };
}

#[test]
fn test_gen_quit() {
    let config = empty_config();
    let generator = Generator::new(config);
    let instruction = generator.gen("quit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_none());
}

#[test]
fn test_gen_exit() {
    let config = empty_config();
    let generator = Generator::new(config);
    let instruction = generator.gen("exit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_none());
}

#[test]
fn test_gen_other() {
    let config = empty_config();
    let generator = Generator::new(config);
    let instruction = generator.gen("ls -la");

    assert!(!instruction.should_terminate);
    assert!(instruction.commands.is_some());

    let commands = instruction.commands.unwrap();
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    assert_eq!(command.program, "ls");
    assert!(command.args.is_some());

    let args = command.args.as_ref().unwrap();
    let arg = args.first().unwrap();
    assert_eq!(arg, "-la");
}
