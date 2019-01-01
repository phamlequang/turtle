#[cfg(test)]
mod test;

use std::cmp::PartialEq;
use std::fmt;

type ExecFn = Box<dyn Fn(&str) -> (bool, String)>;

pub struct Command {
    pub raw: String,
    pub dir: String,
    pub show: bool,
    pub silent: bool,
    pub pipe: bool,
    pub then: Option<ExecFn>,
}

impl Command {
    pub fn new(
        raw: &str,
        dir: &str,
        show: bool,
        silent: bool,
        pipe: bool,
        then: Option<ExecFn>,
    ) -> Self {
        return Self {
            raw: String::from(raw),
            dir: String::from(dir),
            show: show,
            silent: silent,
            pipe: pipe,
            then: then,
        };
    }

    pub fn basic_hide(raw: &str) -> Self {
        return Self::new(raw, "", false, false, false, None);
    }

    pub fn basic_show(raw: &str) -> Self {
        return Self::new(raw, "", true, false, false, None);
    }

    pub fn echo(message: &str) -> Self {
        let raw = format!("echo \"{}\"", message);
        return Self::basic_hide(&raw);
    }
}

impl PartialEq for Command {
    // Check if 2 commands are identical or not
    fn eq(&self, other: &Self) -> bool {
        if self.raw != other.raw
            || self.dir != other.dir
            || self.show != other.show
            || self.silent != other.silent
            || self.pipe != other.pipe
        {
            return false;
        }
        return self.then.is_some() == other.then.is_some();
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let exec = match self.then {
            Some(_) => "Some(closure)",
            None => "None",
        };
        return write!(
            f,
            "Command {{ raw: \"{}\", dir: \"{}\", show: {}, silent: {}, pipe: {}, exec: {} }}",
            self.raw, self.dir, self.show, self.silent, self.pipe, exec,
        );
    }
}
