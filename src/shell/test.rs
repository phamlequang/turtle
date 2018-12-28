use super::*;

#[test]
fn test_home_dir() {
    let home = home_dir();
    assert!(!home.is_empty());
}

#[test]
fn test_normalize_path() {
    let path = "~/Desktop/test.txt";
    let expect = format!("{}/Desktop/test.txt", home_dir());
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
fn test_run_command() {
    let foo = || -> bool {
        println!("hello world!");
        return true;
    };
    let command = Command::new("pwd", "", true, Some(foo));
    let success = run_command(&command);
    assert!(success);
}

#[test]
fn test_run_instruction() {
    let command = Command::basic_show("ls -la");
    let instruction = Instruction::basic(vec![command]);
    let success = run_instruction(&instruction);
    assert!(success);
}
