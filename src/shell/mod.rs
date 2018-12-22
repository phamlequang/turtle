#[cfg(test)]
mod test;

mod cmd;
mod git;
mod instr;

use self::instr::Instruction;
use super::config::*;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";
const CD: &str = "cd";

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

        if let Some(program) = tokens.next() {
            let args: Vec<String> = tokens.map(|t| t.to_owned()).collect();

            match program {
                QUIT | EXIT => return Instruction::terminate(),
                CD => return Instruction::change_directory(args),
                CLONE => return Instruction::clone_repositories(args, &self.config),
                _ => return Instruction::other(program, args),
            }
        }

        return Instruction::do_nothing();
    }
}
