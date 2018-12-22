use super::*;
use crate::config::Config;

#[test]
fn test_generate_terminate_instruction_quit() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("quit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_empty());
}

#[test]
fn test_generate_terminate_instruction_exit() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("exit");

    assert!(instruction.should_terminate);
    assert!(instruction.commands.is_empty());
}

#[test]
fn test_generate_other_instruction() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("ls -la");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    assert_eq!(command.program, "ls");

    let args = &command.args;
    assert_eq!(args.len(), 1);

    let arg = args.first().unwrap();
    assert_eq!(arg, "-la");
}

#[test]
fn test_generate_clone_instruction() {
    let config = Config::load("turtle.toml").unwrap();
    let generator = Generator::new(config);
    let instruction = generator.generate("clone flowers tree");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 2);

    let cmd1 = commands.first().unwrap();
    assert!(cmd1.verbose);
    assert_eq!(cmd1.program, "git");
    assert_eq!(
        cmd1.args,
        vec![
            "clone",
            "git@gitlab.com:phamlequang/flowers.git",
            "/Users/phamlequang/projects/flowers"
        ]
    );

    let cmd2 = commands.last().unwrap();
    assert!(!cmd2.verbose);
    assert_eq!(cmd2.program, "echo");
    assert_eq!(cmd2.args, vec!["--> unknown repository [ tree ]"]);
}
