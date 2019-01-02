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
    let dir = &command.dir;
    if !dir.is_empty() {
        if command.show {
            println!("{}$ cd {}{}", color::Fg(Magenta), dir, style::Reset);
        }
        if let Err(err) = util::change_directory(dir) {
            println!(
                "{}--> cannot change directory to [ {} ]: {}{}",
                color::Fg(Red),
                &command.dir,
                err,
                style::Reset,
            );
            return (false, String::new());
        }
    }

    let raw = &command.raw;
    let mut stdout = String::new();

    if !raw.is_empty() {
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
                        stdout = data.stdout_str();
                    } else {
                        exit_status = data.exit_status;
                        if !command.silent {
                            println!("{}", data.stderr_str());
                        }
                    }
                }
                Err(err) => {
                    exec_error = Some(err);
                }
            }
        } else {
            let result = subprocess::Exec::shell(raw).join();
            match result {
                Ok(status) => {
                    exit_status = status;
                }
                Err(err) => {
                    exec_error = Some(err);
                }
            }
        }

        if !command.silent {
            if let Some(err) = exec_error {
                println!(
                    "{}--> execute error: {}{}",
                    color::Fg(Red),
                    err,
                    style::Reset
                );
                return (false, String::new());
            }

            if !exit_status.success() {
                println!(
                    "{}--> failed with exit status = {:?}{}",
                    color::Fg(Red),
                    exit_status,
                    style::Reset,
                );
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
