use super::*;

#[test]
fn test_run_command_pipe() {
    let exec = |name: &str| -> (bool, String) {
        let s = format!("hello {}", name);
        return (true, s);
    };

    let command = Command::new(
        "echo julia",
        "",
        true,
        false,
        true,
        Some(Box::new(exec)),
        true,
    );

    let (success, output) = run_command(&command);
    assert!(success);
    assert_eq!(output, "hello julia\n");
}

#[test]
fn test_run_command_no_pipe() {
    let exec = |name: &str| -> (bool, String) {
        let s = format!("name = [{}]", name);
        return (true, s);
    };

    let command = Command::new(
        "echo ruby",
        "",
        true,
        false,
        false,
        Some(Box::new(exec)),
        true,
    );

    let (success, output) = run_command(&command);
    assert!(success);
    assert_eq!(output, "name = []");
}

#[test]
fn test_run_instruction() {
    let command = Command::basic_show("ls -la");
    let instruction = Instruction::basic(vec![command]);
    let success = run_instruction(&instruction);
    assert!(success);
}
