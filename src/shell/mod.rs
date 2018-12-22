#[cfg(test)]
mod test;

mod cmd;
mod git;
mod instr;

use self::cmd::Command;
use self::instr::Instruction;
use super::config::*;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";

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

        if let Some(head) = tokens.next() {
            let program = head.to_owned();
            let tail: Vec<String> = tokens.map(|t| t.to_owned()).collect();
            let args = if tail.is_empty() { None } else { Some(tail) };

            match head {
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
        if let Some(names) = args {
            if names.is_empty() {
                return Instruction::do_nothing();
            }

            let mut commands: Vec<Command> = Vec::with_capacity(names.len());
            for name in &names {
                if let Some(repository) = self.config.search_repository(name) {
                    commands.push(git::clone(repository));
                } else {
                    let message = format!("unknown repository {}", name);
                    commands.push(Command::echo(&message));
                }
            }

            return Instruction::normal(commands);
        }

        return Instruction::do_nothing();
    }
}
