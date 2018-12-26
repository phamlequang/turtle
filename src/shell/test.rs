use super::*;

#[test]
fn test_current_directory_shortened() {
    let max_len = 32;
    let dir = current_directory_shortened(max_len);

    let len = dir.len();
    println!("len = {}", len);
    assert!(len >= 1);
    assert!(len <= max_len);
    assert!(dir.ends_with("turtle"));
}

#[test]
fn test_change_directory() {
    let dir = current_directory();
    assert!(!dir.is_empty());
    assert!(dir.ends_with("turtle"));

    let success = change_directory("src");
    assert!(success);

    let dir = current_directory();
    assert!(!dir.is_empty());
    assert!(dir.ends_with("src"));
}

#[test]
fn test_run_command() {
    let command = Command::echo("hello world");
    let success = run_command(&command);
    assert!(success);
}

#[test]
fn test_run_instruction() {
    let command = Command::basic("ls -la");
    let instruction = Instruction::basic(vec![command]);
    let success = run_instruction(&instruction);
    assert!(success);
}
