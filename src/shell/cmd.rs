use std::env;
use std::fmt;
use std::path::Path;
use std::process;
use subprocess;

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub dir: String,
    pub verbose: bool,
}

impl Command {
    pub fn new(program: &str, args: Vec<String>, dir: &str, verbose: bool) -> Self {
        return Self {
            program: program.to_owned(),
            args: args,
            dir: dir.to_owned(),
            verbose: verbose,
        };
    }

    pub fn echo(message: &str) -> Self {
        let args = vec![message.to_owned()];
        return Self::new("echo", args, "", false);
    }

    pub fn display(&self) -> String {
        if self.args.is_empty() {
            return format!("{}", self.program);
        }
        return format!("{} {}", self.program, self.args.join(" "));
    }

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let ok = self.change_directory();

        if !ok || self.program.is_empty() {
            println!();
            return;
        }

        let mut command = process::Command::new(&self.program);
        if !self.args.is_empty() {
            command.args(&self.args);
        }

        let raw = format!("{}", self);
        // if self.verbose {
        println!("$ {}", raw);
        // }

        let result = subprocess::Exec::shell(raw).join();
        match result {
            Ok(status) => {
                if !status.success() {
                    println!("--> failed with exit status = {:?}", status);
                }
            }
            Err(e) => println!("--> error executing command: {}", e),
        }

        // match command.spawn() {
        //     Ok(mut child) => {
        //         if let Err(e) = child.wait() {
        //             println!("failed to wait for child process: {}", e);
        //         }
        //     }
        //     Err(e) => println!("failed to execute command: {}", e),
        // }

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

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.display());
    }
}
