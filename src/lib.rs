use std::io;
use std::io::Write;

pub const QUIT: &str = "quit";
pub const EXIT: &str = "exit";

pub fn run() {
    let mut quit = false;

    while !quit {
        print!("[turtle] > ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        let command = line.to_lowercase();
        let command = command.trim();
        println!("your command is: {}", command);

        match command {
            QUIT | EXIT => quit = true,
            _ => println!("unknown command, please try again"),
        }
    }
}
