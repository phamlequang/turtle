use super::cmd::Command;
use super::docker;
use super::git;
use crate::config::Config;

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
            let command = Command::new("", Vec::new(), &dir, false);
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
                Some(action) => match action.as_ref() {
                    "create" => {
                        let command = docker::create_machine(machine);
                        return Self::new(vec![command], false);
                    }
                    "remove" => {
                        let command = docker::remove_machine(machine);
                        return Self::new(vec![command], false);
                    }
                    "restart" | "env" | "inspect" | "ip" | "kill" | "ls" | "start" | "status"
                    | "stop" | "upgrade" | "url" | "version" => {
                        let command = docker::do_with_machine(action, machine);
                        return Self::new(vec![command], false);
                    }
                    _ => {
                        let message = format!("--> unknown action [ {} ]", action);
                        return Self::echo(&message);
                    }
                },
                None => return Self::do_nothing(),
            }
        }
        return Self::echo("--> docker machine config is missing");
    }

    pub fn other(program: &str, args: Vec<String>) -> Self {
        let command = Command::new(program, args, "", false);
        return Self::new(vec![command], false);
    }

    // Executes all commands sequentially
    pub fn execute(&self) {
        for cmd in &self.commands {
            cmd.execute();
        }
    }
}
