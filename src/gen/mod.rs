#[cfg(test)]
mod test;

use super::config::Config;
use super::docker;
use super::instr::Instruction;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";
const CD: &str = "cd";
const MACHINE: &str = "machine";
const COMPOSE: &str = "compose";

#[derive(Debug)]
pub struct Generator {
    config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        return Self { config };
    }

    pub fn generate_docker_compose_file(&self, file_path: &str) {
        if let Err(e) = docker::generate_compose_file(file_path, &self.config) {
            println!("--> failed to generate docker-compose file: {}", e);
        }
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate_instruction(&self, raw: &str) -> Instruction {
        let mut tokens = raw.trim().split_whitespace();

        if let Some(program) = tokens.next() {
            let args: Vec<String> = tokens.map(|t| t.to_owned()).collect();

            match program {
                QUIT | EXIT => return Instruction::terminate(),
                CD => return Instruction::change_directory(args),
                CLONE => return Instruction::clone_repositories(args, &self.config),
                MACHINE => return Instruction::docker_machine(args, &self.config),
                COMPOSE => return Instruction::docker_compose(args, &self.config),
                _ => return Instruction::other(raw),
            }
        }

        return Instruction::do_nothing();
    }
}
