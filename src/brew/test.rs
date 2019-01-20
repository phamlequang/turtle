use super::*;

#[test]
fn test_install_brew() {
    let command = install_brew();
    let raw = "/usr/bin/ruby -e \"$(curl -fsSL \
               https://raw.githubusercontent.com/Homebrew/install/master/install)\"";

    let expect = Command::basic_show(raw);
    assert_eq!(command, expect);
}

#[test]
fn test_install_packages() {
    let names = ["dnsmasq", "docker"];
    let command = install_packages(&names);

    let raw = "brew install dnsmasq docker";
    let expect = Command::basic_show(raw);
    assert_eq!(command, expect);
}

#[test]
fn test_restart_service() {
    let command = restart_service("mongodb");
    let raw = "brew services restart mongodb";

    let expect = Command::basic_show(&raw);
    assert_eq!(command, expect);
}
