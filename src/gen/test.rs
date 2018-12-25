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
    let expect = Instruction::other(raw);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_clone() {
    let config = Config::default();
    let generator = Generator::new(&config);

    let raw = "clone flowers tree";
    let instruction = generator.generate_instruction(raw);

    let args = vec!["flowers".to_owned(), "tree".to_owned()];
    let expect = Instruction::clone_repositories(args, &config);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_change_directory() {
    let config = Config::new();
    let generator = Generator::new(&config);

    let instruction = generator.generate_instruction("cd ..");
    let expect = Instruction::change_directory(vec!["..".to_owned()]);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_create() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine create");

    let args = vec!["create".to_owned()];
    let expect = Instruction::docker_machine(args, &config);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_remove() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine rm");

    let args = vec!["rm".to_owned()];
    let expect = Instruction::docker_machine(args, &config);

    assert_eq!(instruction, expect);
}

#[test]
fn test_generate_instruction_machine_update_certificates() {
    let config = Config::default();

    let generator = Generator::new(&config);
    let instruction = generator.generate_instruction("machine upcerts");

    let args = vec!["upcerts".to_owned()];
    let expect = Instruction::docker_machine(args, &config);

    assert_eq!(instruction, expect);
}
