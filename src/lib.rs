use std::io;
use std::io::Write;

pub const QUIT: &str = "quit";
pub const EXIT: &str = "exit";

pub fn run() {
    let mut quit = false;

    while !quit {
        let command = prompt("turtle > ");
        quit = execute(&command)
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");

    return line.trim().to_lowercase();
}

fn execute(command: &str) -> bool {
    match command {
        QUIT | EXIT => return true,
        _ => println!("unknown command, please try again"),
    }
    return false;
}
