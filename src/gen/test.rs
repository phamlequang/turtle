use super::*;
use crate::config::Config;

#[test]
fn test_generate_instruction_terminate() {
    let mut config = Config::empty();
    let mut generator = Generator::new(&mut config);

    let expect = Instruction::terminate();

    let instruction = generator.generate_instruction("quit");
    assert_eq!(instruction, expect);

    let instruction = generator.generate_instruction("exit");
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_other() {
    let mut config = Config::empty();
    let mut generator = Generator::new(&mut config);

    let raw = "ls -la";
    let instruction = generator.generate_instruction(raw);

    let command = Command::basic_hide(raw);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_clone() {
    let mut config = Config::default();
    let mut generator = Generator::new(&mut config);

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
    let mut config = Config::empty();
    let mut generator = Generator::new(&mut config);

    let instruction = generator.generate_instruction("cd ..");
    let command = Command::new("", "..", false, None);

    let expect = Instruction::basic(vec![command]);
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_create() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("machine create");

    let machine = &config.docker_machine.unwrap();
    let command = docker::create_machine(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_remove() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("machine rm");

    let machine = &config.docker_machine.unwrap();
    let command = docker::machine_command("rm", machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_update_certificates() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("machine upcerts");

    let machine = &config.docker_machine.unwrap();
    let command = docker::update_certificates(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_load_environment() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("machine load");

    let machine = &config.docker_machine.unwrap();
    let command = docker::load_environments(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_containers() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("docker ps");

    let command = docker::list_containers();
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_images() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("docker images");

    let command = docker::docker_command("images");
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_service_logs() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("logs camellia");

    let machine = &config.docker_machine.unwrap();
    let command = docker::service_logs("camellia", &machine.name);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_use_groups_not_found() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let instruction = generator.generate_instruction("use abcd");

    let expect = Instruction::echo("--> unknown group abcd");
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_use_groups_success() {
    let mut config = Config::default();

    let mut generator = Generator::new(&mut config);
    let _ = generator.generate_instruction("use flowers");

    let use_groups = config.workspace.use_groups.unwrap();
    let expect = vec!["flowers"];
    assert_eq!(use_groups, expect)
}
