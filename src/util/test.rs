use super::*;
use crate::test;

#[test]
fn test_home_directory() {
    let home = home_directory();
    assert!(!home.is_empty());
}

#[test]
fn test_config_directory() {
    let directory = config_directory();
    let expect = format!("{}/.turtle", home_directory());
    assert_eq!(directory, expect);
}

#[test]
fn test_config_file() {
    let file = config_file("/tmp");
    let expect = "/tmp/config.toml";
    assert_eq!(file, expect);
}

#[test]
fn test_compose_file() {
    let file = compose_file("/tmp");
    let expect = "/tmp/docker-compose.yml";
    assert_eq!(file, expect);
}

#[test]
fn test_history_file() {
    let file = history_file("/tmp");
    let expect = "/tmp/.history";
    assert_eq!(file, expect);
}

#[test]
fn test_current_directory() {
    let _lock = test::no_parallel();

    let cur_dir = current_directory();
    assert!(!cur_dir.is_empty());
    assert!(cur_dir.ends_with("turtle"));
}

#[test]
fn test_current_directory_shortened() {
    let _lock = test::no_parallel();

    let max_len = 32;
    let dir = current_directory_shortened(max_len);
    let len = dir.len();
    assert!(len >= 1);
    assert!(len <= max_len);
    assert!(dir.ends_with("turtle"));
}

#[test]
fn test_change_directory() {
    let _lock = test::no_parallel();

    let cur_dir = current_directory();
    assert!(cur_dir.ends_with("turtle"));

    let result = change_directory("~");
    assert!(result.is_ok());

    let dir = current_directory();
    let expect = home_directory();
    assert_eq!(dir, expect);

    let result = change_directory(&cur_dir);
    assert!(result.is_ok());
}

#[test]
fn test_shorten_directory() {
    let directory = "/Users/phamlequang/projects/turtle";
    let max_len = 20;

    let output = shorten_directory(directory, max_len);
    let expect = "projects/turtle";
    assert_eq!(output, expect);
}

#[test]
fn test_normalize_path() {
    let path = "~/Desktop/test.txt";
    let expect = format!("{}/Desktop/test.txt", home_directory());
    assert_eq!(normalize_path(path), expect);
}
