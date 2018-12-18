use serde_derive::{Deserialize, Serialize};
use std::fs;

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
    pub services: Vec<Service>,
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
    pub dependencies: Vec<String>,
    pub repositories: Vec<String>,
}

impl Config {
    pub fn load(file_path: &str) -> Config {
        let content = fs::read_to_string(file_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();
        return config;
    }
}
