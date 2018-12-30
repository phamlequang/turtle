#[cfg(test)]
mod test;

use std::cmp::PartialEq;

use super::cmd::Command;

#[derive(Debug)]
pub struct Instruction {
    pub commands: Vec<Command>,
    pub should_terminate: bool,
}

impl Instruction {
    pub fn new(commands: Vec<Command>, should_terminate: bool) -> Self {
        return Self {
            commands,
            should_terminate,
        };
    }

    pub fn basic(commands: Vec<Command>) -> Self {
        return Self::new(commands, false);
    }

    pub fn skip() -> Self {
        return Self::basic(Vec::new());
    }

    pub fn terminate() -> Self {
        let command = Command::echo("goodbye!");
        return Self::new(vec![command], true);
    }

    pub fn echo(message: &str) -> Self {
        let command = Command::echo(message);
        return Self::basic(vec![command]);
    }
}

impl PartialEq for Instruction {
    // Check if 2 instructions are identical or not
    fn eq(&self, other: &Self) -> bool {
        if self.should_terminate != other.should_terminate {
            return false;
        }

        if self.commands.len() != other.commands.len() {
            return false;
        }

        for (i, cmd1) in self.commands.iter().enumerate() {
            let cmd2 = &other.commands[i];
            if cmd1 != cmd2 {
                return false;
            }
        }

        return true;
    }
}
