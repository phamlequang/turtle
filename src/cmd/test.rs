use super::*;

#[test]
fn test_new_command() {
    let raw = "pwd";
    let dir = "/tmp";
    let show = true;
    let exec = |name: String| -> (String, bool) {
        let out = format!("bonjour {}!", name);
        return (out, true);
    };

    let command = Command::new(raw, dir, show, Some(Box::new(exec)));
    assert_eq!(command.raw, raw);
    assert_eq!(command.dir, dir);
    assert_eq!(command.show, show);
    assert!(command.then.is_some());

    let then = &command.then.unwrap();
    let (out, ok) = then("rust".to_owned());
    assert!(ok);
    assert_eq!(out, "bonjour rust!");
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
