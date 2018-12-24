use std::fs;
use std::io;

use super::cmd::Command;
use crate::config::{Config, Docker, DockerMachine};

pub fn create_machine(machine: &DockerMachine) -> Command {
    let raw = format!(
        "docker-machine create \
         --driver virtualbox \
         --virtualbox-host-dns-resolver \
         --virtualbox-cpu-count {} \
         --virtualbox-disk-size {} \
         --virtualbox-memory {} \
         {}",
        machine.cpu_count, machine.disk_size, machine.memory, machine.name
    );
    return Command::new(&raw, "", true);
}

pub fn update_certificates(machine: &DockerMachine) -> Command {
    let raw = format!(
        "docker-machine regenerate-certs --force --client-certs {}",
        machine.name
    );
    return Command::new(&raw, "", true);
}

pub fn machine_command(action: &str, machine_name: Option<&str>) -> Command {
    let raw = match machine_name {
        Some(name) => format!("docker-machine {} {}", action, name),
        None => format!("docker-machine {}", action),
    };
    return Command::new(&raw, "", true);
}

pub fn compose_command(action: &str, project_name: &str) -> Command {
    let raw = format!("docker-compose --project-name {} {}", project_name, action);
    return Command::new(&raw, "", true);
}

pub fn compose_up(project_name: &str) -> Command {
    return compose_command("up --detach", project_name);
}

pub fn generate_compose_file(file_path: &str, config: &Config) -> io::Result<()> {
    let contents = generate_compose_text(config);
    return fs::write(file_path, contents);
}

pub fn generate_compose_text(config: &Config) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push("version: '3'".to_owned());

    if let Some(volumes) = &config.docker_machine.volumes {
        lines.push(format!("volumes:"));
        for v in volumes {
            lines.push(format!("  {}:", v));
            lines.push(format!("    external: {}", false));
        }
    }

    lines.push("services:".to_owned());

    if let Some(dependencies) = &config.dependencies {
        for dependency in dependencies {
            let more = compose_service(&dependency.name, &dependency.docker);
            lines.extend(more);
        }
    }

    if let Some(respositories) = &config.repositories {
        for repository in respositories {
            if let Some(services) = &repository.services {
                for service in services {
                    let more = compose_service(&service.name, &service.docker);
                    lines.extend(more);
                }
            }
        }
    }

    lines.push(format!(""));

    return lines.join("\n");
}

fn compose_service(name: &str, docker: &Docker) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("  {}:", name));
    lines.push(format!("    image: {}", docker.image));

    if let Some(build) = &docker.build {
        lines.push(format!("    build:"));
        for b in build {
            lines.push(format!("      {}", b));
        }
    }

    if let Some(ports) = &docker.ports {
        lines.push(format!("    ports:"));
        for p in ports {
            lines.push(format!("      - {}", p));
        }
    }

    if let Some(working_dir) = &docker.working_dir {
        lines.push(format!("    working_dir: {}", working_dir));
    }

    if let Some(volumes) = &docker.volumes {
        lines.push(format!("    volumes:"));
        for v in volumes {
            lines.push(format!("      - {}", v));
        }
    }

    if let Some(environment) = &docker.environment {
        lines.push(format!("    environment:"));
        for e in environment {
            lines.push(format!("      - {}", e));
        }
    }

    if let Some(env_file) = &docker.env_file {
        lines.push(format!("    env_file:"));
        for f in env_file {
            lines.push(format!("      - {}", f));
        }
    }

    if let Some(depends_on) = &docker.depends_on {
        lines.push(format!("    depends_on:"));
        for d in depends_on {
            lines.push(format!("      - {}", d));
        }
    }

    if let Some(command) = &docker.command {
        lines.push(format!("    command: {}", command));
    }

    if let Some(labels) = &docker.labels {
        lines.push(format!("    labels:"));
        for l in labels {
            lines.push(format!("      {}", l));
        }
    }

    return lines;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_machine() {
        let machine = DockerMachine::default();

        let command = create_machine(&machine);
        let expect = "docker-machine create \
                      --driver virtualbox \
                      --virtualbox-host-dns-resolver \
                      --virtualbox-cpu-count 2 \
                      --virtualbox-disk-size 16384 \
                      --virtualbox-memory 4096 \
                      turtle";

        assert_eq!(command.raw, expect);
        assert!(command.dir.is_empty());
        assert!(command.show);
    }

    #[test]
    fn test_do_with_machine() {
        let machine = DockerMachine::default();

        let command = machine_command("restart", Some(&machine.name));
        let expect = "docker-machine restart turtle";

        assert_eq!(command.raw, expect);
        assert!(command.dir.is_empty());
        assert!(command.show);
    }

    #[test]
    fn test_update_certificates() {
        let machine = DockerMachine::default();

        let command = update_certificates(&machine);
        let expect = "docker-machine regenerate-certs --force --client-certs turtle";

        assert_eq!(command.raw, expect);
        assert!(command.dir.is_empty());
        assert!(command.show);
    }

    #[test]
    fn test_generate_compose_text() {
        let config = Config::default();
        let result = generate_compose_text(&config);
        let expect = fs::read_to_string("docker-compose.yml").unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_compose_up() {
        let command = compose_up("turtle");
        let expect = "docker-compose --project-name turtle up --detach";

        assert_eq!(command.raw, expect);
        assert!(command.dir.is_empty());
        assert!(command.show);
    }
}