#[cfg(test)]
mod test;

use serde_derive::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fs;
use std::io;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub cpu_count: u32,
    pub disk_size: u32,
    pub memory: u32,
    pub volumes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Docker {
    pub image: String,
    pub build: Option<Vec<String>>,
    pub ports: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub volumes: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub env_file: Option<Vec<String>>,
    pub depends_on: Option<Vec<String>>,
    pub command: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub docker: Docker,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub folder: String,
    pub build: String,
    pub test: String,
    pub docker: Docker,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub remote: String,
    pub local: String,
    pub branch: String,
    pub services: Option<Vec<Service>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub repositories: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project: String,
    pub using: Option<Vec<String>>,
    pub machine: Option<Machine>,
    pub dependencies: Option<Vec<Dependency>>,
    pub repositories: Option<Vec<Repository>>,
    pub groups: Option<Vec<Group>>,
}

impl Config {
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

    pub fn use_groups(&mut self, group_names: Vec<String>) {
        self.using = Some(group_names);
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

    pub fn using_repositories(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        if let Some(using) = &self.using {
            for name in using {
                if let Some(group) = self.search_group(name) {
                    if let Some(repos) = &group.repositories {
                        for repo in repos {
                            result.insert(repo.to_owned());
                        }
                    }
                }
            }
        }

        return result;
    }

    // Return name of all dependencies and services that match the names in args
    // or having their group or repository names that match the names in args
    // Special case: return all if args is empty
    pub fn match_dependencies_and_services(&self, args: Vec<String>) -> HashSet<String> {
        let filters: HashSet<String> = HashSet::from_iter(args);
        let accept_all = filters.is_empty();

        let mut result = HashSet::new();

        if let Some(groups) = &self.groups {
            for group in groups {
                let accept_group = filters.contains(&group.name);

                if let Some(dep_names) = &group.dependencies {
                    for dep_name in dep_names {
                        let accept_dep = filters.contains(dep_name);
                        if accept_all || accept_group || accept_dep {
                            result.insert(dep_name.to_owned());
                        }
                    }
                }

                if let Some(repo_names) = &group.repositories {
                    for repo_name in repo_names {
                        let accept_repo = filters.contains(repo_name);
                        if let Some(repository) = self.search_repository(repo_name) {
                            if let Some(services) = &repository.services {
                                for service in services {
                                    let service_name = service.name.to_owned();
                                    let accept_service = filters.contains(&service_name);
                                    if accept_all || accept_group || accept_repo || accept_service {
                                        result.insert(service_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return result;
    }
}
