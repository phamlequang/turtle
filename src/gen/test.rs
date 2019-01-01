use super::*;

use std::fs;

const CONFIG_DIR: &str = "etc/sample";

fn sample_config() -> Config {
    return Config::load("etc/sample/config.toml").unwrap();
}

#[test]
fn test_generate_instruction_terminate() {
    let mut generator = Generator::new(CONFIG_DIR);

    let expect = Instruction::terminate();

    let instruction = generator.generate_instruction("quit");
    assert_eq!(instruction, expect);

    let instruction = generator.generate_instruction("exit");
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_other() {
    let mut generator = Generator::new(CONFIG_DIR);

    let raw = "ls -la";
    let instruction = generator.generate_instruction(raw);

    let command = Command::basic_hide(raw);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_clone() {
    let config = sample_config();
    let mut generator = Generator::new(CONFIG_DIR);

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
    let mut generator = Generator::new(CONFIG_DIR);

    let instruction = generator.generate_instruction("cd ..");
    let command = Command::new("", "..", false, false, false, None);

    let expect = Instruction::basic(vec![command]);
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_create() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("machine create");

    let machine = &config.machine.unwrap();
    let command = docker::create_machine(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_remove() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("machine rm");

    let machine = &config.machine.unwrap();
    let command = docker::machine_command("rm", machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_update_certificates() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("machine upcerts");

    let machine = &config.machine.unwrap();
    let command = docker::update_certificates(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_load_environment() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("machine load");

    let machine = &config.machine.unwrap();
    let command = docker::load_environments(machine);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_containers() {
    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("docker ps");

    let command = docker::list_containers();
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_list_images() {
    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("docker images");

    let command = docker::docker_command("images");
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_compose() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("compose up -d");

    let command = docker::compose_command("up -d", &config.project, &generator.compose_file);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_docker_service_logs() {
    let config = sample_config();

    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("logs camellia");

    let command = docker::service_logs("camellia", &config.project, &generator.compose_file);
    let expect = Instruction::basic(vec![command]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_restart_services() {
    let config = sample_config();
    let mut generator = Generator::new(CONFIG_DIR);

    let instruction = generator.generate_instruction("restart postgres lotus");
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let service_names = vec!["lotus".to_owned(), "postgres".to_owned()];
    let expect = docker::restart_services(service_names, &config.project, &generator.compose_file);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_generate_instruction_restart_all_services() {
    let config = sample_config();
    let mut generator = Generator::new(CONFIG_DIR);

    let instruction = generator.generate_instruction("restart all");
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let service_names = vec![
        "camellia".to_owned(),
        "lotus".to_owned(),
        "postgres".to_owned(),
        "redis".to_owned(),
    ];
    let expect = docker::restart_services(service_names, &config.project, &generator.compose_file);
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_generate_instruction_use_groups_not_found() {
    let mut generator = Generator::new(CONFIG_DIR);
    let instruction = generator.generate_instruction("use abcd");

    let expect = Instruction::echo("--> unknown group abcd");
    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_use_groups_success() {
    let mut generator = Generator::new("etc/test");
    let instruction = generator.generate_instruction("use dep");

    let message = format!(
        "--> successfully generated new compose file {}",
        &generator.compose_file
    );
    let expect = Instruction::echo(&message);
    assert_eq!(instruction, expect);

    let output_file = "etc/test/docker-compose.yml";
    let expect_file = "etc/test/expect.yml";

    let content = fs::read_to_string(output_file).unwrap();
    let expect = fs::read_to_string(expect_file).unwrap();

    assert_eq!(content, expect);
    fs::remove_file(output_file).unwrap();
}
