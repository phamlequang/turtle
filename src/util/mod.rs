#[cfg(test)]
mod test;

use std::env;
use std::io;
use std::path::{Path, MAIN_SEPARATOR};

const TILDE: &str = "~";

pub fn home_directory() -> String {
    if let Some(pb) = dirs::home_dir() {
        if let Some(dir) = pb.to_str() {
            return String::from(dir);
        }
    }
    return String::new();
}

pub fn default_config_directory() -> String {
    let mut dir = home_directory();
    if dir.len() == 0 {
        dir = String::from("/tmp");
    }
    return format!("{}/.turtle", dir);
}

pub fn config_file(dir: &str, project: &str) -> String {
    return format!("{}/{}.config.toml", dir, project);
}

pub fn compose_file(dir: &str, project: &str) -> String {
    return format!("{}/{}.compose.yml", dir, project);
}

pub fn history_file(dir: &str, project: &str) -> String {
    return format!("{}/{}.history", dir, project);
}

// Change to a specific directory, return true if success
pub fn change_directory(dir: &str) -> io::Result<()> {
    let dir = normalize_path(dir);
    if dir.is_empty() {
        return Ok(());
    }

    let path = Path::new(&dir);
    return env::set_current_dir(path);
}

// Return current directory if success, or empty string if failure
pub fn current_directory() -> String {
    if let Ok(path) = env::current_dir() {
        if let Some(dir) = path.to_str() {
            return String::from(dir);
        }
    }
    return String::new();
}

pub fn current_directory_shortened(max_len: usize) -> String {
    let dir = current_directory();
    if dir.len() <= max_len {
        return dir;
    }
    return shorten_directory(&dir, max_len);
}

pub fn shorten_directory(dir: &str, max_len: usize) -> String {
    let parts = dir.split(MAIN_SEPARATOR);
    let mut dir = String::new();

    for part in parts.rev() {
        let len = dir.len();
        if len == 0 {
            dir.push_str(part);
            continue;
        }

        if len + part.len() >= max_len {
            return dir;
        }
        dir = format!("{}{}{}", part, MAIN_SEPARATOR, dir)
    }

    return dir;
}

pub fn normalize_path(path: &str) -> String {
    let path = path.trim();
    if path.starts_with(TILDE) {
        return format!("{}{}", home_directory(), path.trim_start_matches(TILDE));
    }
    return String::from(path);
}

pub fn path_exist(path: &str) -> bool {
    return Path::new(path).exists();
}
