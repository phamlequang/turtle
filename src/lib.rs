#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

const TURTLE: &str = "turtle > ";
const QUIT: &str = "quit";
const EXIT: &str = "exit";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    docker_machine: DockerMachine,
    dependencies: Option<Vec<Dependency>>,
    repositories: Option<Vec<Repository>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerMachine {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    name: String,
    remote: String,
    local: String,
    branch: String,
    services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    name: String,
    folder: String,
}

impl Config {
    fn new(file_path: &str) -> Config {
        let content = fs::read_to_string(file_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();
        return config;
    }
}

// Run turtle shell
pub fn run() {
    let config = Config::new("turtle.toml");
    println!("{:#?}", config);

    let mut quit = false;
    while !quit {
        let command = prompt(TURTLE);
        quit = execute(&command)
    }
}

// Prompt a message and read a new line from stdin
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    if let Err(e) = io::stdin().read_line(&mut line) {
        println!("failed to read line: {}", e);
    }
    return line;
}

// Execute a command and return true if it is a quit or exit
fn execute(command: &str) -> bool {
    let mut tokens = command.trim().split_whitespace();

    if let Some(program) = tokens.next() {
        match program {
            QUIT | EXIT => return true,
            _ => spawn(program, tokens),
        }
    }

    return false;
}

// Spawn a program as a child process and wait for it to finish
fn spawn<I, S>(program: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = process::Command::new(program);
    command.args(args);

    match command.spawn() {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                println!("failed to wait for child process: {}", e);
            }
        }
        Err(e) => println!("failed to execute command: {}", e),
    }
}
