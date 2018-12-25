use super::*;

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
    let instruction = Instruction::other("ls -la");
    let success = run_instruction(&instruction);
    assert!(success);
}
