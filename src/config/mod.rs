#[cfg(test)]
mod test;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub docker_machine: DockerMachine,
    pub dependencies: Option<Vec<Dependency>>,
    pub repositories: Option<Vec<Repository>>,
    pub groups: Option<Vec<Group>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerMachine {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
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
pub struct Service {
    pub name: String,
    pub folder: String,
    pub build: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub repositories: Option<Vec<String>>,
}

impl Config {
    pub fn load(file_path: &str) -> Result<Config, Error> {
        match fs::read_to_string(file_path) {
            Ok(toml_text) => return Config::parse(&toml_text),
            Err(e) => return Err(e),
        }
    }

    pub fn parse(toml_text: &str) -> Result<Config, Error> {
        match toml::from_str(&toml_text) {
            Ok(config) => return Ok(config),
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
