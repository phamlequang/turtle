#[cfg(test)]
mod test;

use super::cmd::Command;
use super::instr::Instruction;

use dirs;
use std::env;
use std::path::{Path, MAIN_SEPARATOR};
use subprocess::{Exec, Redirection::Pipe};

const TILDE: &str = "~";

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
    let dir = normalize_path(dir);

    if dir.is_empty() {
        return true;
    }

    let path = Path::new(&dir);
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

pub fn normalize_path(path: &str) -> String {
    let path = path.trim();
    if path.starts_with(TILDE) {
        return format!("{}{}", home_dir(), path.trim_start_matches(TILDE));
    }
    return path.to_owned();
}

pub fn home_dir() -> String {
    if let Some(pb) = dirs::home_dir() {
        if let Some(dir) = pb.to_str() {
            return dir.to_owned();
        }
    }
    return String::new();
}

// Execute command as a child process and wait for it to finish,
// return true and output string if success
pub fn run_command(command: &Command) -> (bool, String) {
    let ok = change_directory(&command.dir);
    if !ok {
        return (false, String::new());
    }

    let raw = &command.raw;
    let mut stdout = String::new();

    if !raw.is_empty() {
        if command.show {
            println!("$ {}", raw);
        }

        let exec = subprocess::Exec::shell(raw).stdout(Pipe).stderr(Pipe);
        match exec.capture() {
            Ok(data) => {
                if data.success() {
                    stdout = data.stdout_str();
                } else {
                    let stderr = data.stderr_str();
                    println!("--> failed with stderr = {:?}", stderr);
                    return (false, String::new());;
                }
            }
            Err(err) => {
                println!("--> execute error: {}", err);
                return (false, String::new());
            }
        }
    }

    if let Some(then) = &command.then {
        return then(&stdout);
    }

    return (true, stdout);
}

// Executes all commands sequentially, stop immediately in case of failure,
// return true if success
pub fn run_instruction(instruction: &Instruction) -> bool {
    for cmd in &instruction.commands {
        let (success, _) = run_command(cmd);
        if !success {
            return false;
        }
    }
    return true;
}
