#[derive(Debug)]
pub struct Command {
    pub raw: String,
    pub dir: String,
    pub show: bool,
}

impl Command {
    pub fn new(raw: &str, dir: &str, show: bool) -> Self {
        return Self {
            raw: raw.to_owned(),
            dir: dir.to_owned(),
            show: show,
        };
    }

    pub fn echo(message: &str) -> Self {
        let raw = format!("echo \"{}\"", message);
        return Self::new(&raw, "", false);
    }
}
