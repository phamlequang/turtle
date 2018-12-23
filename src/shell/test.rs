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
    assert_eq!(command.raw, "ls -la");
    assert!(command.dir.is_empty());
    assert!(!command.show);
}

#[test]
fn test_generate_clone_instruction() {
    let config = Config::default();
    let generator = Generator::new(config);
    let instruction = generator.generate("clone flowers tree");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 2);

    let cmd1 = commands.first().unwrap();
    let expect = "git clone \
                  git@gitlab.com:phamlequang/flowers.git \
                  /Users/phamlequang/projects/flowers";
    assert_eq!(cmd1.raw, expect);
    assert!(cmd1.dir.is_empty());
    assert!(cmd1.show);

    let cmd2 = commands.last().unwrap();
    let expect = "echo \"--> unknown repository [ tree ]\"";
    assert_eq!(cmd2.raw, expect);
    assert!(cmd2.dir.is_empty());
    assert!(!cmd2.show);
}

#[test]
fn test_change_directory_instruction() {
    let config = Config::new();
    let generator = Generator::new(config);
    let instruction = generator.generate("cd ..");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    assert!(command.raw.is_empty());
    assert_eq!(command.dir, "..");
    assert!(!command.show);
}

#[test]
fn test_create_docker_machine_instruction() {
    let config = Config::default();
    let generator = Generator::new(config);
    let instruction = generator.generate("machine create");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    let expect = "docker-machine create \
                  --driver virtualbox \
                  --virtualbox-host-dns-resolver \
                  --virtualbox-cpu-count 2 \
                  --virtualbox-disk-size 16384 \
                  --virtualbox-memory 4096 \
                  turtle";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_remove_docker_machine_instruction() {
    let config = Config::default();
    let generator = Generator::new(config);
    let instruction = generator.generate("machine remove");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    let expect = "docker-machine rm turtle";
    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_restart_docker_machine_instruction() {
    let config = Config::default();
    let generator = Generator::new(config);
    let instruction = generator.generate("machine restart");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    let expect = "docker-machine restart turtle";
    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_update_certificates_docker_machine_instruction() {
    let config = Config::default();
    let generator = Generator::new(config);
    let instruction = generator.generate("machine gencerts");

    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let command = commands.first().unwrap();
    let expect = "docker-machine regenerate-certs --force --client-certs turtle";
    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}
