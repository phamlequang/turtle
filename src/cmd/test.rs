use super::*;

fn foo() -> bool {
    println!("test command with exec");
    return true;
}

#[test]
fn test_new_command() {
    let raw = "pwd";
    let dir = "/tmp";
    let show = true;

    let command = Command::new(raw, dir, show, Some(foo));
    let expect = Command {
        raw: raw.to_owned(),
        dir: dir.to_owned(),
        show: show,
        exec: Some(foo),
    };
    assert_eq!(command, expect);
}

#[test]
fn test_basic_command() {
    let raw = "cat test.txt";
    let command = Command::basic_hide(raw);

    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, "");
    assert_eq!(command.show, false);
}

#[test]
fn test_echo_command() {
    let command = Command::echo("hello, i'm turtle");
    let expect = r#"echo "hello, i'm turtle""#;

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(!command.show);
}
