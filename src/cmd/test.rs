use super::*;

#[test]
fn test_new_command() {
    let raw = "pwd";
    let dir = "/tmp";
    let show = true;
    let silent = false;
    let pipe = true;
    let exec = |name: &str| -> (bool, String) {
        let output = format!("bonjour {}!", name);
        return (true, output);
    };
    let back = true;

    let command = Command::new(raw, dir, show, silent, pipe, Some(Box::new(exec)), back);
    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, dir);
    assert_eq!(command.show, show);
    assert_eq!(command.silent, silent);
    assert_eq!(command.pipe, pipe);
    assert!(command.then.is_some());
    assert_eq!(command.back, back);

    let then = &command.then.expect("then is None");
    let (success, output) = then("rust");
    assert!(success);
    assert_eq!(output, "bonjour rust!");
}

#[test]
fn test_basic_command() {
    let raw = "cat test.txt";
    let command = Command::basic_hide(raw);

    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, "");
    assert!(!command.show);
    assert!(!command.pipe);
    assert!(command.then.is_none());
    assert!(command.back);
}

#[test]
fn test_echo_command() {
    let command = Command::echo("hello, i'm turtle");
    let expect = r#"echo "hello, i'm turtle""#;

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(!command.show);
    assert!(!command.pipe);
    assert!(command.then.is_none());
    assert!(command.back);
}
