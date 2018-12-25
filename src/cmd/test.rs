use super::*;

#[test]
fn test_new_command() {
    let raw = "pwd";
    let dir = "/tmp";
    let show = true;

    let command = Command::new(raw, dir, show);
    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, dir);
    assert_eq!(command.show, show);
}

#[test]
fn test_echo_command() {
    let command = Command::echo("hello, i'm turtle");
    let expect = r#"echo "hello, i'm turtle""#;

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(!command.show);
}
