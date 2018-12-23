use std::env;
use std::path::Path;
use subprocess;

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

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let ok = self.change_directory();
        if !ok || self.raw.is_empty() {
            println!();
            return;
        }

        if self.show {
            println!("$ {}", self.raw);
        }

        let result = subprocess::Exec::shell(&self.raw).join();
        match result {
            Ok(status) => {
                if !status.success() {
                    println!("--> failed with exit status = {:?}", status);
                }
            }
            Err(e) => println!("--> execute error: {}", e),
        }

        println!();
    }

    pub fn change_directory(&self) -> bool {
        if self.dir.is_empty() {
            return true;
        }

        let path = Path::new(&self.dir);
        if let Err(e) = env::set_current_dir(path) {
            println!(
                "--> cannot change directory to [ {} ]: {}",
                path.display(),
                e
            );
            return false;
        }

        return true;
    }
}
