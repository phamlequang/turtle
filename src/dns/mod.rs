#[cfg(test)]
mod test;

use super::brew;
use super::cmd::Command;

const DNSMASQ: &str = "dnsmasq";
const CONFIG_FILE: &str = "/usr/local/etc/dnsmasq.conf";
pub const RESOLVER_FOLDER: &str = "/etc/resolver";

pub fn install() -> Command {
    return brew::install_packages(&[DNSMASQ]);
}

pub fn restart() -> Command {
    return brew::restart_service(DNSMASQ);
}

pub fn update(domain: &str, machine: &str) -> Command {
    let raw = format!(
        "sudo tee {} > /dev/null << EOF\n\
         address=/{}/$(docker-machine ip {})\n\
         EOF",
        CONFIG_FILE, domain, machine,
    );
    return Command::basic_show(&raw);
}

pub fn resolve(domain: &str) -> Command {
    let raw = format!(
        "sudo tee {}/{} > /dev/null << EOF\n\
         nameserver 127.0.0.1\n\
         domain {}\n\
         search {}\n\
         search_order 1\n\
         EOF",
        RESOLVER_FOLDER, domain, domain, domain,
    );
    return Command::basic_show(&raw);
}
