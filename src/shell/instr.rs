use super::cmd::Command;
use super::docker;
use super::git;
use crate::config::{Config, DockerMachine};

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

    pub fn do_nothing() -> Self {
        return Self::new(Vec::new(), false);
    }

    pub fn terminate() -> Self {
        return Self::new(Vec::new(), true);
    }

    pub fn echo(message: &str) -> Self {
        let command = Command::echo(message);
        return Self::new(vec![command], false);
    }

    pub fn change_directory(args: Vec<String>) -> Self {
        if let Some(dir) = args.first() {
            let command = Command::new("", &dir, false);
            return Self::new(vec![command], false);
        }
        return Self::do_nothing();
    }

    pub fn clone_repositories(args: Vec<String>, config: &Config) -> Self {
        if args.is_empty() {
            return Self::do_nothing();
        }

        let mut commands: Vec<Command> = Vec::with_capacity(args.len());
        for name in &args {
            if let Some(repository) = config.search_repository(name) {
                commands.push(git::clone(repository));
            } else {
                let message = format!("--> unknown repository [ {} ]", name);
                commands.push(Command::echo(&message));
            }
        }

        return Self::new(commands, false);
    }

    pub fn docker_machine(args: Vec<String>, config: &Config) -> Self {
        if let Some(machine) = &config.docker_machine {
            match args.first() {
                Some(action) => return Self::do_with_docker_machine(action, machine),
                None => return Self::do_nothing(),
            }
        }
        return Self::echo("--> docker machine config is missing");
    }

    pub fn do_with_docker_machine(action: &str, machine: &DockerMachine) -> Instruction {
        match action {
            "create" => {
                let command = docker::create_machine(machine);
                return Self::new(vec![command], false);
            }
            "remove" | "rm" => {
                let command = docker::machine_command("rm", Some(&machine.name));
                return Self::new(vec![command], false);
            }
            "list" | "ls" => {
                let command = docker::machine_command("ls", None);
                return Self::new(vec![command], false);
            }
            "restart" | "env" | "inspect" | "ip" | "kill" | "start" | "status" | "stop"
            | "upgrade" | "url" | "version" => {
                let command = docker::machine_command(action, Some(&machine.name));
                return Self::new(vec![command], false);
            }
            "upcerts" | "gencerts" | "regenerate-certs" => {
                let command = docker::update_certificates(machine);
                return Self::new(vec![command], false);
            }
            "setup" => {
                let commands = vec![
                    docker::create_machine(machine),
                    docker::update_certificates(machine),
                    docker::machine_command("ls", None),
                ];
                return Self::new(commands, false);
            }
            _ => {
                let message = format!("--> unknown action [ {} ]", action);
                return Self::echo(&message);
            }
        }
    }

    pub fn other(raw: &str) -> Self {
        let command = Command::new(raw, "", false);
        return Self::new(vec![command], false);
    }

    // Executes all commands sequentially, stop immediately in case of failure
    pub fn execute(&self) {
        for cmd in &self.commands {
            let success = cmd.execute();
            if !success {
                return;
            }
        }
    }
}
