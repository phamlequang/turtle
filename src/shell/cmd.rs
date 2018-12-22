use std::fmt;
use std::process;

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub verbose: bool,
}

impl Command {
    pub fn new(program: String, args: Option<Vec<String>>, verbose: bool) -> Self {
        return Self {
            program,
            args,
            verbose,
        };
    }

    pub fn with_args(program: String, args: Vec<String>, verbose: bool) -> Self {
        return Self::new(program, Some(args), verbose);
    }

    pub fn echo(message: &str) -> Self {
        let program = String::from("echo");
        let args = vec![message.to_owned()];
        return Self::with_args(program, args, false);
    }

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let mut command = process::Command::new(&self.program);
        if let Some(args) = &self.args {
            command.args(args);
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
        match &self.args {
            Some(args) => write!(f, "{} {}", self.program, args.join(" ")),
            None => write!(f, "{}", self.program),
        }
    }
}
