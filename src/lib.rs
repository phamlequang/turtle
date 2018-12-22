mod config;
mod shell;

use std::io;
use std::io::Write;

const CONFIG_FILE: &str = "turtle.toml";
const TURTLE: &str = "turtle > ";

pub struct Turtle {
    generator: shell::Generator,
}

impl Turtle {
    pub fn new() -> Turtle {
        let config = config::Config::load(CONFIG_FILE).unwrap();
        let generator = shell::Generator::new(config);
        return Turtle { generator };
    }

    // Run turtle shell
    pub fn run(&self) {
        let mut quit = false;
        while !quit {
            let command = self.prompt(TURTLE);
            quit = self.execute(&command)
        }
    }

    // Prompt a message and read a new line from stdin
    fn prompt(&self, message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if let Err(e) = io::stdin().read_line(&mut line) {
            println!("failed to read line: {}", e);
        }
        return line;
    }

    // Execute a command and return true if it is a quit or exit
    fn execute(&self, command: &str) -> bool {
        let instructions = self.generator.gen(command);
        instructions.execute();
        return instructions.terminate;
    }
}
