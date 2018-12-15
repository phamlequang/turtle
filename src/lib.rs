use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::process;

const TURTLE: &str = "turtle > ";
const EMPTY: &str = "";
const QUIT: &str = "quit";
const EXIT: &str = "exit";

// Run turtle shell
pub fn run() {
    let mut quit = false;
    while !quit {
        let command = prompt(TURTLE);
        quit = execute(&command)
    }
}

// Prompt a message to ask for a new command
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => return line.trim().to_owned(),
        Err(e) => {
            println!("failed to read line: {}", e);
            return String::new();
        }
    }
}

// Execute a command
fn execute(command: &str) -> bool {
    match command {
        EMPTY => (),
        QUIT | EXIT => return true,
        _ => system(command),
    }
    return false;
}

// Run any system command as a new child process
fn system(command: &str) {
    let mut tokens = command.split_whitespace();
    if let Some(program) = tokens.next() {
        spawn(program, tokens);
    }
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
