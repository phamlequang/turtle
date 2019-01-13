#[cfg(test)]
mod test;

use super::cmd::Command;
use super::git;
use super::instr::Instruction;
use super::util;

use dirs;
use subprocess::{ExitStatus, PopenError, Redirection::Pipe};
use termion::{color, color::Magenta, color::Red, style};

// Return current git branch of current directory if it is a git repository,
// or empty string if it isn't
pub fn current_git_branch() -> String {
    let command = git::current_branch();

    let (success, branch) = run_command(&command);
    if success {
        return branch;
    }

    return String::new();
}

// Execute command as a child process and wait for it to finish,
// return true and output string if success
pub fn run_command(command: &Command) -> (bool, String) {
    if !change_directory(&command.dir, command.show) {
        return (false, String::new());
    }

    let (success, stdout) = run_raw_command(command);
    if success {
        if let Some(then) = &command.then {
            return then(&stdout);
        }
    }

    return (success, stdout);
}

pub fn run_raw_command(command: &Command) -> (bool, String) {
    let raw = &command.raw;
    if raw.is_empty() {
        return (true, String::new());
    }

    if command.show {
        println!("{}$ {}{}", color::Fg(Magenta), raw, style::Reset);
    }

    let mut exec_error: Option<PopenError> = None;
    let mut exit_status = ExitStatus::Exited(0);

    if command.pipe {
        let exec = subprocess::Exec::shell(raw).stdout(Pipe).stderr(Pipe);

        match exec.capture() {
            Ok(data) => {
                if data.success() {
                    let stdout = data.stdout_str();
                    return (true, stdout);
                } else {
                    exit_status = data.exit_status;
                    if !command.silent {
                        println!("{}", data.stderr_str());
                    }
                }
            }
            Err(err) => exec_error = Some(err),
        }
    } else {
        let result = subprocess::Exec::shell(raw).join();
        match result {
            Ok(status) => exit_status = status,
            Err(err) => exec_error = Some(err),
        }
    }

    return handle_error(command.silent, exec_error, exit_status);
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

pub fn change_directory(dir: &str, show: bool) -> bool {
    if !dir.is_empty() {
        if show {
            println!("{}$ cd {}{}", color::Fg(Magenta), dir, style::Reset);
        }

        if let Err(err) = util::change_directory(dir) {
            println!(
                "{}--> cannot change directory to [ {} ]: {}{}",
                color::Fg(Red),
                dir,
                err,
                style::Reset,
            );
            return false;
        }
    }

    return true;
}

fn handle_error(silent: bool, error: Option<PopenError>, status: ExitStatus) -> (bool, String) {
    if !silent {
        if let Some(err) = error {
            println!(
                "{}--> execute error: {}{}",
                color::Fg(Red),
                err,
                style::Reset
            );
            return (false, String::new());
        }

        if !status.success() {
            println!(
                "{}--> failed with exit status = {:?}{}",
                color::Fg(Red),
                status,
                style::Reset,
            );
            return (false, String::new());
        }
    }

    return (true, String::new());
}
