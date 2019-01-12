#[cfg(test)]
mod test;

use serde_derive::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fs;
use std::io;
use std::iter::FromIterator;

const SERVICE_DIR_PATTERN: &str = "{SERVICE_DIR}";
const REPO_DIR_PATTERN: &str = "{REPO_DIR}";

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub cpu_count: u32,
    pub disk_size: u32,
    pub memory: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerBuild {
    pub context: String,
    pub docker_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Docker {
    pub image: String,
    pub ports: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub volumes: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub env_file: Option<Vec<String>>,
    pub depends_on: Option<Vec<String>>,
    pub command: Option<String>,
    pub labels: Option<Vec<String>>,
    pub build: Option<DockerBuild>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub docker: Docker,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub remote: String,
    pub local: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub name: String,
    pub build: String,
    pub test: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pattern {
    pub format: String,
    pub expand: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub repo: String,
    pub folder: String,
    pub action: String,
    pub docker: Docker,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub services: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project: String,
    pub using: Option<Vec<String>>,
    pub machine: Option<Machine>,
    pub dependencies: Option<Vec<Dependency>>,
    pub repositories: Option<Vec<Repository>>,
    pub actions: Option<Vec<Action>>,
    pub patterns: Option<Vec<Pattern>>,
    pub services: Option<Vec<Service>>,
    pub groups: Option<Vec<Group>>,
}

impl Config {
    pub const SERVICE: usize = 1;
    pub const DEPENDENCY: usize = 2;
    pub const BOTH: usize = Self::SERVICE | Self::DEPENDENCY;

    pub fn load(file_path: &str) -> io::Result<Self> {
        let toml_text = fs::read_to_string(file_path)?;
        return Self::parse(&toml_text);
    }

    pub fn parse(toml_text: &str) -> io::Result<Self> {
        match toml::from_str(&toml_text) {
            Ok(config) => return Ok(config),
            Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
        }
    }

    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let toml_text = self.to_toml()?;
        return fs::write(file_path, toml_text);
    }

    pub fn to_toml(&self) -> io::Result<String> {
        match toml::to_string(&self) {
            Ok(toml_text) => Ok(toml_text),
            Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
        }
    }

    pub fn search_repository(&self, name: &str) -> Option<&Repository> {
        if let Some(repositories) = &self.repositories {
            for repository in repositories {
                if repository.name == name {
                    return Some(repository);
                }
            }
        }
        return None;
    }

    pub fn search_action(&self, name: &str) -> Option<&Action> {
        if let Some(actions) = &self.actions {
            for action in actions {
                if action.name == name {
                    return Some(action);
                }
            }
        }
        return None;
    }

    pub fn search_service(&self, name: &str) -> Option<&Service> {
        if let Some(services) = &self.services {
            for service in services {
                if service.name == name {
                    return Some(service);
                }
            }
        }
        return None;
    }

    pub fn search_service_repository(&self, name: &str) -> Option<&Repository> {
        if let Some(service) = self.search_service(name) {
            return self.search_repository(&service.repo);
        }
        return None;
    }

    pub fn search_service_directory(&self, name: &str) -> Option<String> {
        if let Some(service) = self.search_service(name) {
            return self.service_directory(service);
        }
        return None;
    }

    pub fn service_directory(&self, service: &Service) -> Option<String> {
        if let Some(repository) = self.search_repository(&service.repo) {
            let dir = format!("{}/{}", repository.local, service.folder);
            return Some(dir);
        }
        return None;
    }

    pub fn search_group(&self, name: &str) -> Option<&Group> {
        if let Some(groups) = &self.groups {
            for group in groups {
                if group.name == name {
                    return Some(group);
                }
            }
        }
        return None;
    }

    pub fn use_groups(&mut self, group_names: &[&str]) {
        let using: Vec<String> = group_names.iter().map(|s| String::from(*s)).collect();
        self.using = Some(using);
    }

    pub fn using_dependencies(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        if let Some(using) = &self.using {
            for name in using {
                if let Some(group) = self.search_group(name) {
                    if let Some(deps) = &group.dependencies {
                        for dep in deps {
                            result.insert(dep.to_owned());
                        }
                    }
                }
            }
        }

        return result;
    }

    pub fn using_services(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        if let Some(using) = &self.using {
            for name in using {
                if let Some(group) = self.search_group(name) {
                    if let Some(svcs) = &group.services {
                        for svc in svcs {
                            result.insert(svc.to_owned());
                        }
                    }
                }
            }
        }

        return result;
    }

    pub fn using_repositories(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        let service_names = self.using_services();
        for name in &service_names {
            if let Some(service) = self.search_service(name) {
                result.insert(service.repo.clone());
            }
        }

        return result;
    }

    // Return name of all services and/or dependencies that match the names in args
    // or having their group or repository names that match the names in args
    // Special case: return all if args is empty
    pub fn match_services_dependencies(&self, args: &[&str], choose: usize) -> HashSet<String> {
        let names: Vec<String> = args.iter().map(|s| String::from(*s)).collect();
        let filters: HashSet<String> = HashSet::from_iter(names);
        let accept_all = filters.is_empty();

        let mut result = HashSet::new();

        if let Some(groups) = &self.groups {
            for group in groups {
                let accept_group = filters.contains(&group.name);

                if choose & Self::DEPENDENCY > 0 {
                    if let Some(dep_names) = &group.dependencies {
                        for dep_name in dep_names {
                            let accept_dep = filters.contains(dep_name);
                            if accept_all || accept_group || accept_dep {
                                result.insert(dep_name.to_owned());
                            }
                        }
                    }
                }

                if choose & Self::SERVICE > 0 {
                    if let Some(svc_names) = &group.services {
                        for svc_name in svc_names {
                            let accept_service = filters.contains(svc_name);
                            if let Some(service) = self.search_service(svc_name) {
                                let accept_repo = filters.contains(&service.repo);
                                if accept_all || accept_group || accept_repo || accept_service {
                                    result.insert(svc_name.to_owned());
                                }
                            }
                        }
                    }
                }
            }
        }

        return result;
    }

    pub fn fill_patterns(&self, text: &str, service: Option<&Service>) -> String {
        let mut result = text.to_owned();

        if let Some(patterns) = &self.patterns {
            for pattern in patterns {
                result = result.replace(&pattern.format, &pattern.expand);
            }
        }

        if let Some(service) = service {
            if result.contains(SERVICE_DIR_PATTERN) || result.contains(REPO_DIR_PATTERN) {
                if let Some(repository) = self.search_repository(&service.repo) {
                    let service_dir = format!("{}/{}", repository.local, service.folder);
                    result = result.replace(SERVICE_DIR_PATTERN, &service_dir);
                    result = result.replace(REPO_DIR_PATTERN, &repository.local);
                }
            }
        }

        return result;
    }
}
