#[cfg(test)]
mod test;

use std::cmp::PartialEq;

type ExecFn = fn() -> bool;

#[derive(Debug)]
pub struct Command {
    pub raw: String,
    pub dir: String,
    pub show: bool,
    pub exec: Option<ExecFn>,
}

impl Command {
    pub fn new(raw: &str, dir: &str, show: bool, exec: Option<ExecFn>) -> Self {
        return Self {
            raw: raw.to_owned(),
            dir: dir.to_owned(),
            show: show,
            exec: exec,
        };
    }

    pub fn basic_hide(raw: &str) -> Self {
        return Self::new(&raw, "", false, None);
    }

    pub fn basic_show(raw: &str) -> Self {
        return Self::new(&raw, "", true, None);
    }

    pub fn echo(message: &str) -> Self {
        let raw = format!("echo \"{}\"", message);
        return Self::basic_hide(&raw);
    }
}

impl PartialEq for Command {
    // Check if 2 commands are identical or not
    fn eq(&self, other: &Self) -> bool {
        if self.raw != other.raw || self.dir != other.dir || self.show != other.show {
            return false;
        }
        match self.exec {
            Some(exec1) => {
                if let Some(exec2) = other.exec {
                    return exec1 as usize == exec2 as usize;
                }
                return false;
            }
            None => {
                if let None = other.exec {
                    return true;
                }
                return false;
            }
        }
    }
}
