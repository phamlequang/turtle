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

    assert!(command.dir.is_empty());
    assert!(!command.verbose);
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
    assert_eq!(cmd1.program, "git");
    assert_eq!(
        cmd1.args,
        vec![
            "clone",
            "git@gitlab.com:phamlequang/flowers.git",
            "/Users/phamlequang/projects/flowers"
        ]
    );
    assert!(cmd1.dir.is_empty());
    assert!(cmd1.verbose);

    let cmd2 = commands.last().unwrap();
    assert_eq!(cmd2.program, "echo");
    assert_eq!(cmd2.args, vec!["--> unknown repository [ tree ]"]);
    assert!(cmd2.dir.is_empty());
    assert!(!cmd2.verbose);
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
    assert!(command.program.is_empty());
    assert_eq!(command.dir, "..");
    assert!(!command.verbose);
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

    assert_eq!(command.display(), expect);
    assert_eq!(command.program, "docker-machine");
    assert_eq!(command.args.len(), 11);
    assert!(command.dir.is_empty());
    assert!(command.verbose);
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

    assert_eq!(command.display(), expect);
    assert_eq!(command.program, "docker-machine");
    assert_eq!(command.args.len(), 2);
    assert!(command.dir.is_empty());
    assert!(command.verbose);
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

    assert_eq!(command.display(), expect);
    assert_eq!(command.program, "docker-machine");
    assert_eq!(command.args.len(), 2);
    assert!(command.dir.is_empty());
    assert!(command.verbose);
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

    assert_eq!(command.display(), expect);
    assert_eq!(command.program, "docker-machine");
    assert_eq!(command.args.len(), 4);
    assert!(command.dir.is_empty());
    assert!(command.verbose);
}
