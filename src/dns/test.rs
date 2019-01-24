use super::*;

#[test]
fn test_install_dns() {
    let command = install();
    let expect = brew::install_packages(&["dnsmasq"]);
    assert_eq!(command, expect);
}

#[test]
fn test_restart_dns() {
    let command = restart();
    let expect = brew::restart_service("dnsmasq");
    assert_eq!(command, expect);
}

#[test]
fn test_update_dns() {
    let command = update("dev.turtle.co", "turtle");
    let raw = "sudo tee /usr/local/etc/dnsmasq.conf > /dev/null << EOF\n\
               address=/dev.turtle.co/$(docker-machine ip turtle)\n\
               EOF";

    let expect = Command::basic_show(&raw);
    assert_eq!(command, expect);
}

#[test]
fn test_resolve_dns() {
    let command = resolve("dev.turtle.co");
    let raw = "sudo tee /etc/resolver/dev.turtle.co > /dev/null << EOF\n\
               nameserver 127.0.0.1\n\
               domain dev.turtle.co\n\
               search dev.turtle.co\n\
               search_order 1\n\
               EOF";

    let expect = Command::basic_show(&raw);
    assert_eq!(command, expect);
}
