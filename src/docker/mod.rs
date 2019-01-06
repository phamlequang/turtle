#[cfg(test)]
mod test;

use std::fs;
use std::io;

use super::cmd::Command;
use super::config::{Config, Docker, Machine, Service};

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

pub fn compose_command(action: &str, project: &str, compose_file: &str) -> Command {
    let raw = format!(
        "docker-compose -p {} -f {} {}",
        project, compose_file, action
    );
    return Command::basic_show(&raw);
}

pub fn service_logs(service_name: &str, project: &str, compose_file: &str) -> Command {
    let action = format!("logs -f --tail=100 {}", service_name);
    return compose_command(&action, project, compose_file);
}

pub fn restart_services(services: &[&str], project: &str, compose_file: &str) -> Command {
    let action = format!("restart {}", services.join(" "));
    return compose_command(&action, project, compose_file);
}

pub fn stop_services(service_names: &[&str], project: &str, compose_file: &str) -> Command {
    let action = format!("stop {}", service_names.join(" "));
    return compose_command(&action, project, compose_file);
}

pub fn status_services(project: &str, compose_file: &str) -> Command {
    let action = "ps";
    return compose_command(&action, project, compose_file);
}

pub fn compose_exec(service: &str, cmd: &str, project: &str, compose_file: &str) -> Command {
    let action = format!("exec {} {}", service, cmd);
    return compose_command(&action, project, compose_file);
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
    let mut volumes: Vec<String> = Vec::new();

    lines.push(String::from("version: '3'"));
    lines.push(String::from("services:"));

    let using_dependencies = config.using_dependencies();
    let using_services = config.using_services();

    if let Some(dependencies) = &config.dependencies {
        for dependency in dependencies {
            if !using_dependencies.contains(&dependency.name) {
                continue;
            }
            let (more_lines, more_volumes) =
                compose_service(&dependency.name, &dependency.docker, config, None);
            lines.extend(more_lines);
            volumes.extend(more_volumes);
        }
    }

    if let Some(services) = &config.services {
        for service in services {
            if !using_services.contains(&service.name) {
                continue;
            }
            let (more_lines, more_volumes) =
                compose_service(&service.name, &service.docker, config, Some(service));
            lines.extend(more_lines);
            volumes.extend(more_volumes);
        }
    }

    if !volumes.is_empty() {
        lines.push(format!("volumes:"));
        for v in volumes {
            lines.push(format!("  {}:", v));
            lines.push(format!("    external: {}", false));
        }
    }

    lines.push(format!(""));

    return lines;
}

fn compose_service(
    name: &str,
    docker: &Docker,
    config: &Config,
    service: Option<&Service>,
) -> (Vec<String>, Vec<String>) {
    let mut lines: Vec<String> = Vec::new();
    let mut named_volumes: Vec<String> = Vec::new();

    lines.push(format!("  {}:", name));
    lines.push(format!("    image: {}", docker.image));

    if let Some(build) = &docker.build {
        lines.push(format!("    build:"));

        let context = config.fill_patterns(&build.context, service);
        lines.push(format!("      context: {}", context));

        if let Some(docker_file) = &build.docker_file {
            let docker_file = config.fill_patterns(docker_file, service);
            lines.push(format!("      dockerfile: {}", docker_file));
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
            let volume = config.fill_patterns(v, service);
            lines.push(format!("      - {}", volume));

            if let Some(named_volume) = extract_named_volume(&volume) {
                named_volumes.push(named_volume);
            }
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
            let file = config.fill_patterns(f, service);
            lines.push(format!("      - {}", file));
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
            lines.push(format!("      - {}", l));
        }
    }

    return (lines, named_volumes);
}

fn extract_named_volume(name: &str) -> Option<String> {
    let tokens: Vec<&str> = name.split(":").collect();
    if tokens.len() != 2 {
        return None;
    }

    let volume = tokens[0].to_owned();
    let prefixes = ["/", ".", "~"];

    for p in &prefixes {
        if volume.starts_with(p) {
            return None;
        }
    }

    return Some(volume);
}
