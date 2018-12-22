#[cfg(test)]
mod test;

use super::config::Config;
use std::process;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";

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

#[derive(Debug)]
pub struct Instruction {
    pub commands: Option<Vec<Command>>,
    pub should_terminate: bool,
}

impl Instruction {
    pub fn new(commands: Option<Vec<Command>>, should_terminate: bool) -> Self {
        return Self {
            commands,
            should_terminate,
        };
    }

    pub fn do_nothing() -> Self {
        return Self::new(None, false);
    }

    pub fn terminate() -> Self {
        return Self::new(None, true);
    }

    pub fn normal(commands: Vec<Command>) -> Self {
        return Self::new(Some(commands), false);
    }

    // Executes all commands sequentially
    pub fn execute(&self) {
        if let Some(commands) = self.commands.as_ref() {
            for cmd in commands {
                cmd.execute();
            }
        }
    }
}

#[derive(Debug)]
pub struct Generator {
    config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        return Self { config };
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate(&self, raw: &str) -> Instruction {
        let mut tokens = raw.trim().split_whitespace();

        if let Some(first) = tokens.next() {
            let program = first.to_owned();
            let tokens: Vec<String> = tokens.map(|t| t.to_owned()).collect();
            let args = if tokens.is_empty() {
                None
            } else {
                Some(tokens)
            };

            match first {
                QUIT | EXIT => return self.terminate_instruction(),
                CLONE => return self.clone_instruction(args),
                _ => return self.other_instruction(program, args),
            }
        }

        return Instruction::do_nothing();
    }

    fn terminate_instruction(&self) -> Instruction {
        return Instruction::terminate();
    }

    fn other_instruction(&self, program: String, args: Option<Vec<String>>) -> Instruction {
        let command = Command::new(program, args);
        return Instruction::new(Some(vec![command]), false);
    }

    fn clone_instruction(&self, args: Option<Vec<String>>) -> Instruction {
        if let Some(repos) = args {
            if repos.is_empty() {
                return Instruction::do_nothing();
            }

            let mut commands: Vec<Command> = Vec::with_capacity(repos.len());

            for name in &repos {
                let result = self.config.search_repository(name);
                if let Some(repository) = result {
                    let program = String::from("git");
                    let args = vec![
                        String::from("clone"),
                        repository.remote.clone(),
                        repository.local.clone(),
                    ];
                    let command: Command = Command {
                        program: program,
                        args: Some(args),
                    };
                    commands.push(command);
                }
            }

            return Instruction::normal(commands);
        }

        return Instruction::do_nothing();
    }
}
