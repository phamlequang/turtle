use super::*;

#[test]
fn test_home_directory() {
    let home = home_directory();
    assert!(!home.is_empty());
}

#[test]
fn test_config_directory() {
    let directory = config_directory();
    let expect = format!("{}/.turtle", home_directory());
    assert_eq!(directory, expect);
}

#[test]
fn test_normalize_path() {
    let path = "~/Desktop/test.txt";
    let expect = format!("{}/Desktop/test.txt", home_directory());
    assert_eq!(normalize_path(path), expect);
}

#[test]
fn test_change_and_current_directory() {
    let dir = current_directory();
    assert!(!dir.is_empty());
    assert!(dir.ends_with("turtle"));

    let success = change_directory("src");
    assert!(success);

    let max_len = 32;
    let dir = current_directory_shortened(max_len);
    let len = dir.len();
    assert!(len >= 1);
    assert!(len <= max_len);
    assert!(dir.ends_with("src"));
}

#[test]
fn test_run_command_pipe() {
    let exec = |name: &str| -> (bool, String) {
        let s = format!("hello {}", name);
        return (true, s);
    };

    let command = Command::new("echo julia", "", true, false, true, Some(Box::new(exec)));
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

    let command = Command::new("echo ruby", "", true, false, false, Some(Box::new(exec)));
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
