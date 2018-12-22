mod config;
mod shell;

use std::io;
use std::io::Write;

const CONFIG_FILE: &str = "turtle.toml";
const TURTLE: &str = "turtle > ";

// Run turtle shell
pub fn run() {
    let config = config::Config::load(CONFIG_FILE).unwrap();
    let generator = shell::Generator::new(config);

    let mut done = false;

    while !done {
        let line = prompt(TURTLE);

        let instruction = generator.generate(&line);
        instruction.execute();

        done = instruction.should_terminate;
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
