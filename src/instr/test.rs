use super::*;

#[test]
fn test_skip() {
    let instruction = Instruction::skip();
    assert!(instruction.commands.is_empty());
    assert!(!instruction.should_terminate);
}

#[test]
fn test_terminate() {
    let instruction = Instruction::terminate();
    assert!(instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::echo("goodbye!");
    assert_eq!(&commands[0], &expect);
}

#[test]
fn test_echo() {
    let message = "this is a test";

    let instruction = Instruction::echo(message);
    assert!(!instruction.should_terminate);

    let commands = &instruction.commands;
    assert_eq!(commands.len(), 1);

    let expect = Command::echo(message);
    assert_eq!(&commands[0], &expect);
}
