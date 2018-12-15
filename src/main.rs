use std::io;
use std::io::Write;

fn main() {
    let mut quit = false;

    while !quit {
        print!("[turtle] > ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        let command = line.trim();
        println!("your command is: {}", command);

        match command {
            "quit" | "exit" => quit = true,
            _ => println!("unknown command, please try again"),
        }
    }
}
