#[cfg(test)]
mod test;

use super::cmd::Command;
use super::instr::Instruction;

use std::env;
use std::path::Path;
use subprocess;

use rustyline::Editor;

// Return current directory if success, or empty string if failure
pub fn current_directory() -> String {
    if let Ok(path) = env::current_dir() {
        if let Some(s) = path.to_str() {
            return s.to_owned();
        }
    }
    return "".to_owned();
}

// Change to a specific directory, return true if success
pub fn change_directory(dir: &str) -> bool {
    if dir.is_empty() {
        return true;
    }

    let path = Path::new(dir);
    if let Err(err) = env::set_current_dir(path) {
        println!(
            "--> cannot change directory to [ {} ]: {}",
            path.display(),
            err
        );
        return false;
    }

    return true;
}

// Execute command as a child process and wait for it to finish, return true if success
pub fn run_command(command: &Command) -> bool {
    let ok = change_directory(&command.dir);
    let raw = &command.raw;

    if !ok || raw.is_empty() {
        println!();
        return true;
    }

    if command.show {
        println!("\n$ {}", raw);
    }
    println!();

    let result = subprocess::Exec::shell(raw).join();
    match result {
        Ok(status) => {
            if status.success() {
                println!();
                return true;
            }
            println!("--> failed with exit status = {:?}\n", status);
        }
        Err(err) => {
            println!("--> execute error: {}\n", err);
        }
    }

    return false;
}

// Executes all commands sequentially, stop immediately in case of failure, return true if success
pub fn run_instruction(instruction: &Instruction) -> bool {
    for cmd in &instruction.commands {
        let success = run_command(cmd);
        if !success {
            return false;
        }
    }
    return true;
}

// Prompt current directory and read a new line from stdin
pub fn prompt(editor: &mut Editor<()>) -> String {
    let message = format!("{} ~ ", current_directory());

    match editor.readline(&message) {
        Ok(line) => {
            editor.add_history_entry(line.as_ref());
            return line;
        }
        Err(_) => return String::new(),
    }
}
