#[cfg(test)]
mod test;

use serde_derive::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub cpu_count: u32,
    pub disk_size: u32,
    pub memory: u32,
    pub volumes: Option<Vec<String>>,
}

impl Machine {
    #[cfg(test)]
    pub fn default() -> Self {
        Self {
            name: String::from("turtle"),
            cpu_count: 2,
            disk_size: 16384,
            memory: 4096,
            volumes: None,
        }
    }
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
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            project: String::new(),
            using: None,
            machine: None,
            dependencies: None,
            repositories: None,
            groups: None,
        }
    }

    #[cfg(test)]
    pub fn default() -> Self {
        return Self::load("turtle.toml").unwrap();
    }

    pub fn load(file_path: &str) -> Result<Self, io::Error> {
        match fs::read_to_string(file_path) {
            Ok(toml_text) => return Self::parse(&toml_text),
            Err(err) => return Err(err),
        }
    }

    pub fn parse(toml_text: &str) -> Result<Self, io::Error> {
        match toml::from_str(&toml_text) {
            Ok(config) => return Ok(config),
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
                            result.insert(dep.clone());
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
                            result.insert(repo.clone());
                        }
                    }
                }
            }
        }

        return result;
    }
}
