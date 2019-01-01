#[cfg(test)]
mod test;

use super::cmd::Command;
use super::config::Config;
use super::docker;
use super::git;
use super::instr::Instruction;
use super::util;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const CLONE: &str = "clone";
const CD: &str = "cd";
const MACHINE: &str = "machine";
const COMPOSE: &str = "compose";
const DOCKER: &str = "docker";
const LOGS: &str = "logs";
const USE: &str = "use";
const START: &str = "start";
const STOP: &str = "stop";
const RESTART: &str = "restart";
const STATUS: &str = "status";

#[derive(Debug)]
pub struct Generator {
    config: Config,
    config_file: String,
    compose_file: String,
}

impl Generator {
    pub fn new(config_dir: &str) -> Generator {
        let config_file = util::config_file(&config_dir);
        let compose_file = util::compose_file(&config_dir);
        let config: Config;

        match Config::load(&config_file) {
            Ok(cfg) => config = cfg,
            Err(err) => {
                panic!("-> cannot load config file {}: {}", config_file, err);
            }
        }

        return Self {
            config,
            config_file,
            compose_file,
        };
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate_instruction(&mut self, raw: &str) -> Instruction {
        let mut tokens = raw.trim().split_whitespace();

        if let Some(program) = tokens.next() {
            let args: Vec<String> = tokens.map(|t| t.to_owned()).collect();

            match program {
                QUIT | EXIT => return self.terminate(),
                CD => return self.change_directory(args),
                CLONE => return self.clone_repositories(args),
                MACHINE => return self.machine(args),
                COMPOSE => return self.docker_compose(args),
                DOCKER => return self.docker(args),
                LOGS => return self.service_logs(args),
                USE => return self.use_groups(args),
                START => return self.start_services(),
                STOP => return self.stop_services(args),
                RESTART => return self.restart_services(args),
                STATUS => return self.status_services(),
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
            let command = Command::new("", &dir, false, false, false, None);
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

    fn machine(&self, args: Vec<String>) -> Instruction {
        match &self.config.machine {
            Some(machine) => {
                if let Some(action) = args.first() {
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
            }
            None => return Instruction::echo("docker machine config is not found"),
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
        if args.is_empty() {
            return Instruction::skip();
        }
        let action = args.join(" ");
        let command = docker::compose_command(&action, &self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn service_logs(&self, args: Vec<String>) -> Instruction {
        if let Some(service_name) = args.first() {
            let command =
                docker::service_logs(service_name, &self.config.project, &self.compose_file);
            return Instruction::basic(vec![command]);
        }
        return Instruction::skip();
    }

    fn start_services(&self) -> Instruction {
        let command = docker::compose_command("up -d", &self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn stop_services(&self, args: Vec<String>) -> Instruction {
        if args.len() == 0 {
            let command = docker::compose_command("down", &self.config.project, &self.compose_file);
            return Instruction::basic(vec![command]);
        }

        let matches = self.config.match_dependencies_and_services(args);
        let mut services: Vec<String> = matches.into_iter().collect();
        services.sort();

        let command = docker::stop_services(services, &self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn restart_services(&self, args: Vec<String>) -> Instruction {
        let matches = self.config.match_dependencies_and_services(args);
        let mut services: Vec<String> = matches.into_iter().collect();
        services.sort();

        let command = docker::restart_services(services, &self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn status_services(&self) -> Instruction {
        let command = docker::status_services(&self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn use_groups(&mut self, args: Vec<String>) -> Instruction {
        if args.len() == 0 {
            return Instruction::skip();
        }

        for name in &args {
            if let None = self.config.search_group(&name) {
                let message = format!("--> unknown group {}", name);
                return Instruction::echo(&message);
            }
        }

        self.config.use_groups(args);

        if let Err(err) = docker::generate_compose_file(&self.compose_file, &self.config) {
            let message = format!(
                "--> cannot generate compose file {}: {}",
                &self.compose_file, err
            );
            return Instruction::echo(&message);
        }

        if let Err(err) = self.config.save(&self.config_file) {
            let message = format!("--> cannot save config file {}: {}", &self.config_file, err);
            return Instruction::echo(&message);
        }

        let message = format!(
            "--> successfully generated new compose file {} and save config file {}",
            &self.compose_file, &self.config_file,
        );
        return Instruction::echo(&message);
    }

    fn other(&self, raw: &str) -> Instruction {
        let command = Command::basic_hide(raw);
        return Instruction::basic(vec![command]);
    }
}
