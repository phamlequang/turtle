use std::process;

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Option<Vec<String>>,
}

impl Command {
    pub fn new(program: String, args: Option<Vec<String>>) -> Self {
        return Self { program, args };
    }

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let mut command = process::Command::new(&self.program);
        if let Some(args) = self.args.as_ref() {
            command.args(args);
        }

        match command.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    println!("failed to wait for child process: {}", e);
                }
            }
            Err(e) => println!("failed to execute command: {}", e),
        }
    }
}
