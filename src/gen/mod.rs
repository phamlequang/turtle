#[cfg(test)]
mod test;

use super::brew;
use super::cmd::Command;
use super::config::Config;
use super::dns;
use super::docker;
use super::git;
use super::instr::Instruction;
use super::util;

const QUIT: &str = "quit";
const EXIT: &str = "exit";
const INSTALL: &str = "install";
const CD: &str = "cd";
const GOTO: &str = "goto";
const CLONE: &str = "clone";
const PULL: &str = "pull";
const PUSH: &str = "push";
const MACHINE: &str = "dkmc";
const COMPOSE: &str = "dkcp";
const DOCKER: &str = "dk";
const LOGS: &str = "logs";
const USE: &str = "use";
const START: &str = "start";
const STOP: &str = "stop";
const RESTART: &str = "restart";
const STATUS: &str = "status";
const BASH: &str = "bash";
const SH: &str = "sh";
const BUILD: &str = "build";
const TEST: &str = "test";
const DNS: &str = "dns";

#[derive(Debug)]
pub struct Generator {
    config: Config,
    config_file: String,
    compose_file: String,
}

impl Generator {
    pub fn new(config_dir: &str, project: &str) -> Result<Generator, String> {
        let config_file = util::config_file(config_dir, project);
        let compose_file = util::compose_file(config_dir, project);

        let config = if util::path_exist(&config_file) {
            match Config::load(&config_file) {
                Ok(cfg) => cfg,
                Err(err) => {
                    let msg = format!(
                        "cannot load config file [ {} ]: {}",
                        config_file, err
                    );
                    return Err(msg);
                }
            }
        } else {
            let cfg = Config::new(project);

            if let Err(err) = cfg.save(&config_file) {
                let msg = format!(
                    "cannot save config file [ {} ]: {}",
                    config_file, err
                );
                return Err(msg);
            }

            println!("--> new config file was generated: [ {} ]", config_file);
            cfg
        };

        let generator = Self {
            config,
            config_file,
            compose_file,
        };
        return Ok(generator);
    }

    // Takes a raw instruction string, returns a list of instructions to execute
    pub fn generate_instruction(&mut self, raw: &str) -> Instruction {
        let mut raw = util::normalize_spaces(raw);
        if let Some(shortcuts) = &self.config.shortcuts {
            raw = util::replace_shortcuts(&raw, shortcuts);
        }

        let mut tokens = raw.trim().split_whitespace();

        if let Some(program) = tokens.next() {
            let args: Vec<&str> = tokens.collect();

            match program {
                QUIT | EXIT => return self.terminate(),
                INSTALL => return self.install(&args),
                CD => return self.change_directory(&args),
                GOTO => return self.goto(&args),
                CLONE => return self.clone_repositories(&args),
                PULL => return self.pull_repositories(&args),
                PUSH => return self.push_repositories(&args),
                MACHINE => return self.machine(&args),
                COMPOSE => return self.docker_compose(&args),
                DOCKER => return self.docker(&args),
                LOGS => return self.service_logs(&args),
                USE => return self.use_groups(&args),
                START => return self.start_services(),
                STOP => return self.stop_services(&args),
                RESTART => return self.restart_services(&args),
                STATUS => return self.status_services(),
                BASH | SH => return self.open_service_shell(&program, &args),
                BUILD | TEST => return self.do_services(&program, &args),
                DNS => return self.do_dns(&args),
                _ => return self.other(&raw),
            }
        }

        return Instruction::skip();
    }

    fn terminate(&self) -> Instruction {
        return Instruction::terminate();
    }

    fn install(&self, args: &[&str]) -> Instruction {
        let command = if args.contains(&"brew") {
            brew::install_brew()
        } else {
            brew::install_packages(args)
        };
        return Instruction::basic(vec![command]);
    }

    fn change_directory(&self, args: &[&str]) -> Instruction {
        if let Some(dir) = args.first() {
            let command =
                Command::new("", &dir, false, false, false, None, false);
            return Instruction::basic(vec![command]);
        }
        return Instruction::skip();
    }

    fn goto(&self, args: &[&str]) -> Instruction {
        if let Some(name) = args.first() {
            if let Some(service_dir) =
                self.config.search_service_directory(name)
            {
                return self.change_directory(&[&service_dir]);
            }

            if let Some(repository) = self.config.search_repository(name) {
                return self.change_directory(&[&repository.local]);
            }

            let message =
                format!("--> unknown service or repository [ {} ]", name);
            return Instruction::echo(&message);
        }

        return Instruction::skip();
    }

    fn clone_repositories(&self, args: &[&str]) -> Instruction {
        if args.is_empty() {
            let repos = self.config.using_repositories();
            if repos.is_empty() {
                return Instruction::echo("--> nothing to clone");
            }

            let repos: Vec<&str> = repos.iter().map(String::as_ref).collect();
            return self.clone_repositories(&repos);
        }

        let mut commands: Vec<Command> = Vec::with_capacity(args.len());
        for name in args {
            if let Some(repository) = self.config.search_repository(name) {
                commands.push(git::clone_repository(repository));
            } else {
                let message = format!("--> unknown repository [ {} ]", name);
                commands.push(Command::echo(&message));
            }
        }

        return Instruction::basic(commands);
    }

    fn pull_repositories(&self, args: &[&str]) -> Instruction {
        return self.git_do_repositories(args, git::pull_repository);
    }

    fn push_repositories(&self, args: &[&str]) -> Instruction {
        return self.git_do_repositories(args, git::push_repository);
    }

    fn git_do_repositories(
        &self,
        args: &[&str],
        doit: fn(&str) -> Command,
    ) -> Instruction {
        if args.is_empty() {
            let command = doit("");
            return Instruction::basic(vec![command]);
        }

        let mut commands: Vec<Command> = Vec::with_capacity(args.len());
        for name in args {
            if let Some(repository) = self.config.search_repository(name) {
                commands.push(doit(&repository.local));
            } else if let Some(repository) =
                self.config.search_service_repository(name)
            {
                commands.push(doit(&repository.local));
            } else {
                let message =
                    format!("--> unknown repository or service [ {} ]", name);
                commands.push(Command::echo(&message));
            }
        }

        return Instruction::basic(commands);
    }

    fn machine(&self, args: &[&str]) -> Instruction {
        match &self.config.machine {
            Some(machine) => {
                if let Some(action) = args.first() {
                    match *action {
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
                            let command =
                                docker::machine_command(&raw, machine);
                            return Instruction::basic(vec![command]);
                        }
                    }
                }
            }
            None => {
                return Instruction::echo(
                    "--> docker machine config is not found",
                )
            }
        }
        return Instruction::skip();
    }

    fn docker(&self, args: &[&str]) -> Instruction {
        if let Some(action) = args.first() {
            match *action {
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

    fn docker_compose(&self, args: &[&str]) -> Instruction {
        if args.is_empty() {
            return Instruction::skip();
        }
        let action = args.join(" ");
        let command = docker::compose_command(
            &action,
            &self.config.project,
            &self.compose_file,
        );
        return Instruction::basic(vec![command]);
    }

    fn service_logs(&self, args: &[&str]) -> Instruction {
        if let Some(service_name) = args.first() {
            let command = docker::service_logs(
                service_name,
                &self.config.project,
                &self.compose_file,
            );
            return Instruction::basic(vec![command]);
        }
        return Instruction::skip();
    }

    fn start_services(&self) -> Instruction {
        let command = docker::compose_command(
            "up -d",
            &self.config.project,
            &self.compose_file,
        );
        return Instruction::basic(vec![command]);
    }

    fn stop_services(&self, args: &[&str]) -> Instruction {
        if args.len() == 0 {
            let command = docker::compose_command(
                "down",
                &self.config.project,
                &self.compose_file,
            );
            return Instruction::basic(vec![command]);
        }

        let matches =
            self.config.match_services_dependencies(args, Config::BOTH);
        let mut services: Vec<_> = matches.iter().map(String::as_ref).collect();
        services.sort();

        let command = docker::stop_services(
            &services,
            &self.config.project,
            &self.compose_file,
        );
        return Instruction::basic(vec![command]);
    }

    fn restart_services(&self, args: &[&str]) -> Instruction {
        let matches =
            self.config.match_services_dependencies(args, Config::BOTH);
        let mut services: Vec<_> = matches.iter().map(String::as_ref).collect();
        services.sort();

        let command = docker::restart_services(
            &services,
            &self.config.project,
            &self.compose_file,
        );
        return Instruction::basic(vec![command]);
    }

    fn status_services(&self) -> Instruction {
        let command =
            docker::status_services(&self.config.project, &self.compose_file);
        return Instruction::basic(vec![command]);
    }

    fn open_service_shell(
        &self,
        shell_type: &str,
        args: &[&str],
    ) -> Instruction {
        match shell_type {
            BASH | SH => {
                if let Some(service) = args.first() {
                    let cmd = if shell_type == SH { "/bin/sh" } else { BASH };
                    let command = docker::compose_exec(
                        service,
                        cmd,
                        &self.config.project,
                        &self.compose_file,
                    );
                    return Instruction::basic(vec![command]);
                }
                return Instruction::echo("--> service name is not provided");
            }
            _ => {
                let message =
                    format!("--> unknown shell type [ {} ]", shell_type);
                return Instruction::echo(&message);
            }
        }
    }

    fn do_services(&self, what: &str, args: &[&str]) -> Instruction {
        let matches = self
            .config
            .match_services_dependencies(args, Config::SERVICE);

        let mut svc_names: Vec<_> =
            matches.iter().map(String::as_ref).collect();
        svc_names.sort();

        let mut commands: Vec<Command> = Vec::new();

        for svc_name in svc_names {
            if let Some(service) = self.config.search_service(svc_name) {
                if let Some(action) = self.config.search_action(&service.action)
                {
                    if let Some(dir) = self.config.service_directory(service) {
                        let raw = match what {
                            BUILD => &action.build,
                            TEST => &action.test,
                            _ => "",
                        };

                        if raw.len() == 0 {
                            continue;
                        }

                        let command = Command::new(
                            raw, &dir, true, false, false, None, true,
                        );
                        commands.push(command);
                    }
                }
            }
        }

        return Instruction::basic(commands);
    }

    fn use_groups(&mut self, args: &[&str]) -> Instruction {
        if args.len() == 0 {
            return Instruction::skip();
        }

        for name in args {
            if let None = self.config.search_group(&name) {
                let message = format!("--> unknown group [ {} ]", name);
                return Instruction::echo(&message);
            }
        }

        self.config.use_groups(args);

        if let Err(err) =
            docker::generate_compose_file(&self.compose_file, &self.config)
        {
            let message = format!(
                "--> cannot generate compose file [ {} ]: {}",
                &self.compose_file, err
            );
            return Instruction::echo(&message);
        }

        if let Err(err) = self.config.save(&self.config_file) {
            let message = format!(
                "--> cannot save config file [ {} ]: {}",
                &self.config_file, err
            );
            return Instruction::echo(&message);
        }

        let message = format!(
            "--> saved compose: [ {} ] and config: [ {} ]",
            &self.compose_file, &self.config_file,
        );
        return Instruction::echo(&message);
    }

    fn do_dns(&self, args: &[&str]) -> Instruction {
        if let Some(action) = args.first() {
            let mut commands: Vec<Command> = Vec::new();
            match *action {
                "install" => commands.push(dns::install()),
                "restart" => commands.push(dns::restart()),
                "update" | "resolve" => {
                    if let Some(machine) = &self.config.machine {
                        if *action == "update" {
                            commands
                                .push(dns::update(&machine.dns, &machine.name));
                        } else {
                            let raw = format!(
                                "sudo mkdir -p {}",
                                dns::RESOLVER_FOLDER
                            );
                            commands.push(Command::basic_show(&raw));
                            commands.push(dns::resolve(&machine.dns));
                        };
                    } else {
                        let message = "--> docker machine config doesn't exist";
                        commands.push(Command::echo(message));
                    }
                }
                _ => {
                    let message = format!("--> unsupported action {}", action);
                    commands.push(Command::echo(&message));
                }
            };
            return Instruction::basic(commands);
        }

        return Instruction::skip();
    }

    fn other(&self, raw: &str) -> Instruction {
        let command = Command::basic_hide(raw);
        return Instruction::basic(vec![command]);
    }
}
