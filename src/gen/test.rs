use super::*;
use crate::config::Config;

#[test]
fn test_generate_instruction_terminate() {
    let config = Config::new();
    let generator = Generator::new(&config);

    let expect = Instruction::terminate();

    let instruction = generator.generate_instruction("quit");
    assert_eq!(instruction, expect);

    let instruction = generator.generate_instruction("exit");
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_other() {
    let config = Config::new();
    let generator = Generator::new(&config);

    let raw = "ls -la";
    let instruction = generator.generate_instruction(raw);

    let command = Command::basic(raw);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_clone() {
    let config = Config::default();
    let generator = Generator::new(&config);

    let raw = "clone flowers tree";
    let instruction = generator.generate_instruction(raw);

    let repository = config.search_repository("flowers").unwrap();
    let cmd1 = git::clone_repository(repository);
    let cmd2 = Command::echo("--> unknown repository [ tree ]");

    let expect = Instruction::basic(vec![cmd1, cmd2]);
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_change_directory() {
    let config = Config::new();
    let generator = Generator::new(&config);

    let instruction = generator.generate_instruction("cd ..");
    let command = Command::new("", "..", false);

    let expect = Instruction::basic(vec![command]);
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_create() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine create");

    let command = docker::create_machine(&config.docker_machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_remove() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine rm");

    let command = docker::machine_command("rm", &config.docker_machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_update_certificates() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine upcerts");

    let command = docker::update_certificates(&config.docker_machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_load_environment() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine load");

    let command = docker::load_environments(&config.docker_machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_containers() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("docker ps");

    let command = docker::list_containers();
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_images() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("docker images");

    let command = docker::docker_command("images");
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_service_logs() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("logs camellia");

    let command = docker::service_logs("camellia", &config.docker_machine.name);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}
