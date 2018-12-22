use std::fmt;
use std::process;

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub verbose: bool,
}

impl Command {
    pub fn new(program: String, args: Vec<String>, verbose: bool) -> Self {
        return Self {
            program,
            args,
            verbose,
        };
    }

    pub fn echo(message: &str) -> Self {
        let program = String::from("echo");
        let args = vec![message.to_owned()];
        return Self::new(program, args, false);
    }

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let mut command = process::Command::new(&self.program);
        if !self.args.is_empty() {
            command.args(&self.args);
        }

        if self.verbose {
            println!("$ {}", self);
        }

        match command.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    println!("failed to wait for child process: {}", e);
                }
            }
            Err(e) => println!("failed to execute command: {}", e),
        }

        println!();
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.args.is_empty() {
            return write!(f, "{}", self.program);
        }
        return write!(f, "{} {}", self.program, self.args.join(" "));
    }
}
