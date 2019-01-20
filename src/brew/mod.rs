#[cfg(test)]
mod test;

use super::cmd::Command;

const INSTALL_BREW: &str = "/usr/bin/ruby -e \"$(curl -fsSL \
                            https://raw.githubusercontent.com/Homebrew/install/master/install)\"";

pub fn install_brew() -> Command {
    return Command::basic_show(INSTALL_BREW);
}

pub fn install_packages(names: &[&str]) -> Command {
    let raw = format!("brew install {}", names.join(" "));
    return Command::basic_show(&raw);
}
