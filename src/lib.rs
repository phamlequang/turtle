mod config;
mod shell;

use std::env;
use std::io;
use std::io::Write;

const CONFIG_FILE: &str = "turtle.toml";

// Run turtle shell
pub fn run() {
    let config = config::Config::load(CONFIG_FILE).unwrap();
    let generator = shell::Generator::new(config);

    let mut stop = false;
    while !stop {
        let line = prompt();

        let instruction = generator.generate(&line);
        instruction.execute();

        stop = instruction.should_terminate;
    }
}

// Prompt current directory and read a new line from stdin
fn prompt() -> String {
    if let Ok(path) = env::current_dir() {
        print!("{} ", path.display());
    }
    print!("~ ");

    io::stdout().flush().unwrap();

    let mut line = String::new();
    if let Err(e) = io::stdin().read_line(&mut line) {
        println!("failed to read line: {}", e);
    }

    return line;
}
