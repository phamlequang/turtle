#[cfg(test)]
mod test;

use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Command {
    pub raw: String,
    pub dir: String,
    pub show: bool,
}

impl PartialEq for Command {
    // Check if 2 commands are identical or not
    fn eq(&self, other: &Self) -> bool {
        return self.raw == other.raw && self.dir == other.dir && self.show == other.show;
    }
}

impl Command {
    pub fn new(raw: &str, dir: &str, show: bool) -> Self {
        return Self {
            raw: raw.to_owned(),
            dir: dir.to_owned(),
            show: show,
        };
    }

    pub fn basic(raw: &str) -> Self {
        return Self::new(&raw, "", false);
    }

    pub fn echo(message: &str) -> Self {
        let raw = format!("echo \"{}\"", message);
        return Self::new(&raw, "", false);
    }
}
