#[cfg(test)]
mod test;

use super::cmd::Command;
use super::instr::Instruction;

use std::env;
use std::path::{Path, MAIN_SEPARATOR};
use subprocess::{Exec, Redirection::Pipe};

// Return current directory if success, or empty string if failure
pub fn current_directory() -> String {
    if let Ok(path) = env::current_dir() {
        if let Some(s) = path.to_str() {
            return s.to_owned();
        }
    }
    return "".to_owned();
}

pub fn current_directory_shortened(max_len: usize) -> String {
    let dir = current_directory();
    if dir.len() <= max_len {
        return dir;
    }

    let parts = dir.split(MAIN_SEPARATOR);
    let mut dir = String::new();

    for p in parts.rev() {
        let len = dir.len();
        if len == 0 {
            dir = p.to_owned();
            continue;
        }

        if len + p.len() >= max_len {
            return dir;
        }
        dir = format!("{}{}{}", p, MAIN_SEPARATOR, dir)
    }

    return dir;
}

// Return current git branch of current directory if it is a git repository,
// or empty string if it isn't
pub fn current_git_branch() -> String {
    let exec = Exec::shell("git branch").stdout(Pipe).stderr(Pipe);

    if let Ok(data) = exec.capture() {
        if data.success() {
            for branch in data.stdout_str().lines() {
                if branch.starts_with("*") {
                    return branch.to_owned();
                }
            }
        }
    }

    return String::new();
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
