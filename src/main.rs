use std::io;
use std::io::Write;

const QUIT: &str = "quit";
const EXIT: &str = "exit";

struct Turtle {}

impl Turtle {
    fn run(&self) {
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
}

fn main() {
    let turtle = Turtle {};
    turtle.run();
}
