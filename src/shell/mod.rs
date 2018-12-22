#[cfg(test)]
mod test;

use super::config::Config;
use std::ffi::OsStr;
use std::process;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";

#[derive(Debug)]
pub struct Command<S>
where
    S: AsRef<OsStr>,
{
    pub program: S,
    pub args: Option<Vec<S>>,
}

impl<S> Command<S>
where
    S: AsRef<OsStr>,
{
    pub fn new(program: S, args: Option<Vec<S>>) -> Self
    where
        S: AsRef<OsStr>,
    {
        return Self { program, args };
    }

    // Execute command as a child process and wait for it to finish
    pub fn execute(&self) {
        let mut command = process::Command::new(self.program.as_ref());
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

pub struct Instruction<S>
where
    S: AsRef<OsStr>,
{
    pub commands: Option<Vec<Command<S>>>,
    pub should_terminate: bool,
}

impl<S> Instruction<S>
where
    S: AsRef<OsStr>,
{
    pub fn new(commands: Option<Vec<Command<S>>>, should_terminate: bool) -> Self {
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

    pub fn normal(commands: Vec<Command<S>>) -> Self {
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

pub struct Generator {
    config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        return Self { config };
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate(&self, raw: &str) -> Instruction<String> {
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

    fn terminate_instruction(&self) -> Instruction<String> {
        return Instruction::terminate();
    }

    fn other_instruction(&self, program: String, args: Option<Vec<String>>) -> Instruction<String> {
        let command: Command<String> = Command::new(program, args);
        return Instruction::new(Some(vec![command]), false);
    }

    fn clone_instruction(&self, args: Option<Vec<String>>) -> Instruction<String> {
        if let Some(repos) = args {
            if repos.is_empty() {
                return Instruction::do_nothing();
            }

            let mut commands: Vec<Command<String>> = Vec::with_capacity(repos.len());

            for name in &repos {
                let result = self.config.search_repository(name);
                if let Some(repository) = result {
                    let program = String::from("git");
                    let args = vec![
                        String::from("clone"),
                        repository.remote.clone(),
                        repository.local.clone(),
                    ];
                    let command: Command<String> = Command {
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
