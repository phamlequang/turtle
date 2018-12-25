#[cfg(test)]
mod test;

use std::cmp::PartialEq;

use super::cmd::Command;
use super::config::Config;
use super::docker;
use super::git;

#[derive(Debug)]
pub struct Instruction {
    pub commands: Vec<Command>,
    pub should_terminate: bool,
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
        let command = Command::echo("goodbye!");
        return Self::new(vec![command], true);
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
                commands.push(git::clone_repository(repository));
            } else {
                let message = format!("--> unknown repository [ {} ]", name);
                commands.push(Command::echo(&message));
            }
        }

        return Self::new(commands, false);
    }

    pub fn docker_machine(args: Vec<String>, config: &Config) -> Self {
        if let Some(action) = args.first() {
            let machine = &config.docker_machine;
            match action.as_ref() {
                "create" => {
                    let command = docker::create_machine(machine);
                    return Self::new(vec![command], false);
                }
                "upcerts" => {
                    let command = docker::update_certificates(machine);
                    return Self::new(vec![command], false);
                }
                "start" => {
                    let commands = vec![
                        docker::machine_command("start", machine),
                        docker::load_environments(&machine),
                    ];
                    return Self::new(commands, false);
                }
                _ => {
                    let raw = args.join(" ");
                    let command = docker::machine_command(&raw, machine);
                    return Self::new(vec![command], false);
                }
            }
        }
        return Self::do_nothing();
    }

    pub fn docker_compose(args: Vec<String>, config: &Config) -> Self {
        if !args.is_empty() {
            let project_name = &config.docker_machine.name;
            let action = args.join(" ");
            let command = docker::compose_command(&action, &project_name);
            return Self::new(vec![command], false);
        }
        return Self::do_nothing();
    }

    pub fn other(raw: &str) -> Self {
        let command = Command::new(raw, "", false);
        return Self::new(vec![command], false);
    }
}
