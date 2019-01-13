use super::*;

#[test]
fn test_run_command_pipe() {
    let exec = |name: &str| -> (bool, String) {
        let s = format!("hello {}", name);
        return (true, s);
    };

    let command = Command::new(
        "echo julia",
        "..",
        true,
        false,
        true,
        Some(Box::new(exec)),
        false,
    );

    let prev_dir = util::current_directory();
    let (success, output) = run_command(&command);

    assert!(success);
    assert_eq!(output, "hello julia\n");

    let back_dir = util::current_directory();
    assert_ne!(back_dir, prev_dir);

    let result = util::change_directory(&prev_dir);
    assert!(result.is_ok());
}

#[test]
fn test_run_command_no_pipe() {
    let exec = |name: &str| -> (bool, String) {
        let s = format!("name = [{}]", name);
        return (true, s);
    };

    let command = Command::new(
        "echo ruby",
        "..",
        true,
        false,
        false,
        Some(Box::new(exec)),
        true,
    );

    let prev_dir = util::current_directory();
    let (success, output) = run_command(&command);

    assert!(success);
    assert_eq!(output, "name = []");

    let back_dir = util::current_directory();
    assert_eq!(back_dir, prev_dir);
}

#[test]
fn test_run_instruction() {
    let command = Command::basic_show("ls -la");
    let instruction = Instruction::basic(vec![command]);
    let success = run_instruction(&instruction);
    assert!(success);
}
