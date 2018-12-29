#[cfg(test)]
mod test;

use std::fs;
use std::io;

use super::cmd::Command;
use super::config::{Config, Docker, Machine};

pub fn create_machine(machine: &Machine) -> Command {
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
    return Command::basic_show(&raw);
}

pub fn update_certificates(machine: &Machine) -> Command {
    let raw = format!(
        "docker-machine regenerate-certs --force --client-certs {}",
        machine.name
    );
    return Command::basic_show(&raw);
}

pub fn load_environments(machine: &Machine) -> Command {
    let raw = format!("eval \"$(docker-machine env {})\"", machine.name);
    return Command::basic_show(&raw);
}

pub fn machine_command(action: &str, machine: &Machine) -> Command {
    let raw = format!("docker-machine {} {}", action, machine.name);
    return Command::basic_show(&raw);
}

pub fn compose_command(action: &str, project_name: &str) -> Command {
    let raw = format!("docker-compose -p {} {}", project_name, action);
    return Command::basic_show(&raw);
}

pub fn service_logs(service_name: &str, project_name: &str) -> Command {
    let action = format!("logs -f --tail=100 {}", service_name);
    return compose_command(&action, project_name);
}

pub fn docker_command(action: &str) -> Command {
    let raw = format!("docker {}", action);
    return Command::basic_hide(&raw);
}

pub fn list_containers() -> Command {
    let action = "ps -a --format \"table \
                  {{.Names}}\t{{.Image}}\t{{.Size}}\t\
                  {{.CreatedAt}}\t{{.Status}}\"";
    return docker_command(action);
}

pub fn generate_compose_file(file_path: &str, config: &Config) -> io::Result<()> {
    let contents = generate_compose_text(config);
    return fs::write(file_path, contents);
}

pub fn generate_compose_text(config: &Config) -> String {
    let lines = generate_compose_lines(config);
    return lines.join("\n");
}

pub fn generate_compose_lines(config: &Config) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    match &config.machine {
        Some(machine) => {
            lines.push("version: '3'".to_owned());

            if let Some(volumes) = &machine.volumes {
                lines.push(format!("volumes:"));
                for v in volumes {
                    lines.push(format!("  {}:", v));
                    lines.push(format!("    external: {}", false));
                }
            }

            lines.push("services:".to_owned());

            let using_dependencies = config.using_dependencies();
            let using_repositories = config.using_repositories();

            if let Some(dependencies) = &config.dependencies {
                for dependency in dependencies {
                    if !using_dependencies.contains(&dependency.name) {
                        continue;
                    }
                    let more = compose_service(&dependency.name, &dependency.docker);
                    lines.extend(more);
                }
            }

            if let Some(respositories) = &config.repositories {
                for repository in respositories {
                    if !using_repositories.contains(&repository.name) {
                        continue;
                    }
                    if let Some(services) = &repository.services {
                        for service in services {
                            let more = compose_service(&service.name, &service.docker);
                            lines.extend(more);
                        }
                    }
                }
            }

            lines.push(format!(""));

            return lines;
        }
        None => return lines,
    }
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
