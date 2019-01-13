use super::*;

#[test]
fn test_red() {
    let output = red("red");
    let expect = format!("{}red{}", color::Fg(color::Red), style::Reset);
    assert_eq!(output, expect);
}

#[test]
fn test_green() {
    let output = green("green");
    let expect = format!("{}green{}", color::Fg(color::Green), style::Reset);
    assert_eq!(output, expect);
}

#[test]
fn test_yellow() {
    let output = yellow("yellow");
    let expect = format!("{}yellow{}", color::Fg(color::Yellow), style::Reset);
    assert_eq!(output, expect);
}
