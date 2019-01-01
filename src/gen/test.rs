use super::*;

use std::fs;

const CONFIG_DIR: &str = "etc";
const CONFIG_FILE: &str = "etc/config.toml";
const TEST_CONFIG_DIR: &str = "etc/test";
const TEST_CONFIG_FILE: &str = "etc/test/config.toml";
const TEST_COMPOSE_FILE: &str = "etc/test/compose.yml";
const TEST_EXPECT_FILE: &str = "etc/expect.yml";

fn sample_config() -> Config {
    return Config::load(CONFIG_FILE).unwrap();
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
    fs::create_dir_all(TEST_CONFIG_DIR).expect("failed to create test config directory");
    fs::copy(CONFIG_FILE, TEST_CONFIG_FILE).expect("failed to copy config file to test directory");

    let mut generator = Generator::new(TEST_CONFIG_DIR);
    let instruction = generator.generate_instruction("use dep");

    let message = format!(
        "--> successfully generated new compose file {} and save config file {}",
        TEST_COMPOSE_FILE, TEST_CONFIG_FILE,
    );
    let expect = Instruction::echo(&message);
    assert_eq!(instruction, expect);

    let content = fs::read_to_string(TEST_COMPOSE_FILE).expect("failed to read compose file");
    let expect = fs::read_to_string(TEST_EXPECT_FILE).expect("failed to read expect file");
    assert_eq!(content, expect);

    let using = generator.config.using.expect("using field is None");
    assert_eq!(using, vec![String::from("dep")]);

    fs::remove_dir_all(TEST_CONFIG_DIR).expect("failed to remove test config directory");
}
