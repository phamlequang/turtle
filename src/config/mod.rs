#[cfg(test)]
mod test;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerMachine {
    pub name: String,
    pub cpu_count: u32,
    pub disk_size: u32,
    pub memory: u32,
}

impl DockerMachine {
    #[cfg(test)]
    pub fn default() -> Self {
        DockerMachine {
            name: String::from("turtle"),
            cpu_count: 2,
            disk_size: 10240,
            memory: 4096,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub folder: String,
    pub build: String,
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
    pub docker_machine: Option<DockerMachine>,
    pub dependencies: Option<Vec<Dependency>>,
    pub repositories: Option<Vec<Repository>>,
    pub groups: Option<Vec<Group>>,
}

impl Config {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            docker_machine: None,
            dependencies: None,
            repositories: None,
            groups: None,
        }
    }

    #[cfg(test)]
    pub fn default() -> Self {
        return Self::load("turtle.toml").unwrap();
    }

    pub fn load(file_path: &str) -> Result<Self, Error> {
        match fs::read_to_string(file_path) {
            Ok(toml_text) => return Self::parse(&toml_text),
            Err(e) => return Err(e),
        }
    }

    pub fn parse(toml_text: &str) -> Result<Self, Error> {
        match toml::from_str(&toml_text) {
            Ok(config) => return Ok(config),
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
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
}
