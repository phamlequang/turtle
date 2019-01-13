#[cfg(test)]
mod test;

use termion::{color, style};

pub fn red(text: &str) -> String {
    return format!("{}{}{}", color::Fg(color::Red), text, style::Reset);
}

pub fn green(text: &str) -> String {
    return format!("{}{}{}", color::Fg(color::Green), text, style::Reset);
}

pub fn yellow(text: &str) -> String {
    return format!("{}{}{}", color::Fg(color::Yellow), text, style::Reset);
}
