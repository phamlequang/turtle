use super::*;
use crate::config::Config;

#[test]
fn test_generate_terminate_instruction_quit() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("quit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_none());
}

#[test]
fn test_generate_terminate_instruction_exit() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("exit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_none());
}

#[test]
fn test_generate_other_instruction() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("ls -la");

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

#[test]
fn test_generate_clone_instruction() {
    let config = Config::load("turtle.toml").unwrap();
    let generator = Generator::new(config);
    let instruction = generator.generate("clone flowers tree");

    assert!(!instruction.should_terminate);
    assert!(instruction.commands.is_some());

    let commands = instruction.commands.unwrap();
    assert_eq!(commands.len(), 2);
}
