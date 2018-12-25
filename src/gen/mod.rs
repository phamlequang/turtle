#[cfg(test)]
mod test;

use super::cmd::Command;
use super::config::Config;
use super::docker;
use super::git;
use super::instr::Instruction;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";
const CD: &str = "cd";
const MACHINE: &str = "machine";
const COMPOSE: &str = "compose";
const DOCKER: &str = "docker";
const LOGS: &str = "logs";

#[derive(Debug)]
pub struct Generator<'a> {
    config: &'a Config,
}

impl<'a> Generator<'a> {
    pub fn new(config: &'a Config) -> Generator {
        return Self { config };
    }

    pub fn generate_docker_compose_file(&self, file_path: &str) {
        if let Err(err) = docker::generate_compose_file(file_path, &self.config) {
            println!("--> failed to generate docker-compose file: {}", err);
        }
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate_instruction(&self, raw: &str) -> Instruction {
        let mut tokens = raw.trim().split_whitespace();

        if let Some(program) = tokens.next() {
            let args: Vec<String> = tokens.map(|t| t.to_owned()).collect();

            match program {
                QUIT | EXIT => return self.terminate(),
                CD => return self.change_directory(args),
                CLONE => return self.clone_repositories(args),
                MACHINE => return self.docker_machine(args),
                COMPOSE => return self.docker_compose(args),
                DOCKER => return self.docker(args),
                LOGS => return self.service_logs(args),
                _ => return self.other(raw),
            }
        }

        return Instruction::skip();
    }

    fn terminate(&self) -> Instruction {
        return Instruction::terminate();
    }

    fn change_directory(&self, args: Vec<String>) -> Instruction {
        if let Some(dir) = args.first() {
            let command = Command::new("", &dir, false);
            return Instruction::new(vec![command], false);
        }
        return Instruction::skip();
    }

    fn clone_repositories(&self, args: Vec<String>) -> Instruction {
        if args.is_empty() {
            return Instruction::skip();
        }

        let mut commands: Vec<Command> = Vec::with_capacity(args.len());
        for name in &args {
            if let Some(repository) = self.config.search_repository(name) {
                commands.push(git::clone_repository(repository));
            } else {
                let message = format!("--> unknown repository [ {} ]", name);
                commands.push(Command::echo(&message));
            }
        }

        return Instruction::basic(commands);
    }

    fn docker_machine(&self, args: Vec<String>) -> Instruction {
        if let Some(action) = args.first() {
            let machine = &self.config.docker_machine;
            match action.as_ref() {
                "create" => {
                    let command = docker::create_machine(machine);
                    return Instruction::basic(vec![command]);
                }
                "upcerts" => {
                    let command = docker::update_certificates(machine);
                    return Instruction::basic(vec![command]);
                }
                "load" => {
                    let command = docker::load_environments(&machine);
                    return Instruction::basic(vec![command]);
                }
                _ => {
                    let raw = args.join(" ");
                    let command = docker::machine_command(&raw, machine);
                    return Instruction::basic(vec![command]);
                }
            }
        }
        return Instruction::skip();
    }

    fn docker(&self, args: Vec<String>) -> Instruction {
        if let Some(action) = args.first() {
            match action.as_ref() {
                "ps" => {
                    let command = docker::list_containers();
                    return Instruction::basic(vec![command]);
                }
                _ => {
                    let raw = args.join(" ");
                    let command = docker::docker_command(&raw);
                    return Instruction::basic(vec![command]);
                }
            }
        }
        return Instruction::skip();
    }

    fn docker_compose(&self, args: Vec<String>) -> Instruction {
        if !args.is_empty() {
            let project_name = &self.config.docker_machine.name;
            let action = args.join(" ");
            let command = docker::compose_command(&action, &project_name);
            return Instruction::basic(vec![command]);
        }
        return Instruction::skip();
    }

    fn service_logs(&self, args: Vec<String>) -> Instruction {
        if let Some(service_name) = args.first() {
            let project_name = &self.config.docker_machine.name;
            let command = docker::service_logs(service_name, project_name);
            return Instruction::basic(vec![command]);
        }
        return Instruction::skip();
    }

    fn other(&self, raw: &str) -> Instruction {
        let command = Command::new(raw, "", false);
        return Instruction::basic(vec![command]);
    }
}
