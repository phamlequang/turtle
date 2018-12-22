#[cfg(test)]
mod test;

use super::config::Config;
use std::ffi::OsStr;
use std::process;

const QUIT: &str = "quit";
const EXIT: &str = "exit";

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

    // Executes all commands sequentially
    pub fn execute(&self) {
        if let Some(commands) = self.commands.as_ref() {
            for cmd in commands {
                cmd.execute()
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
    pub fn gen(&self, raw: &str) -> Instruction<String> {
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
                QUIT | EXIT => return Instruction::terminate(),
                _ => return self.other(program, args),
            }
        }

        return Instruction::do_nothing();
    }

    fn other<S>(&self, program: S, args: Option<Vec<S>>) -> Instruction<S>
    where
        S: AsRef<OsStr>,
    {
        let command: Command<S> = Command::new(program, args);
        return Instruction::new(Some(vec![command]), false);
    }
}
