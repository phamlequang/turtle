use super::*;

#[test]
fn test_home_directory() {
    let home = home_directory();
    assert!(!home.is_empty());
}

#[test]
fn test_default_config_directory() {
    let directory = default_config_directory();
    let expect = format!("{}/.turtle", home_directory());
    assert_eq!(directory, expect);
}

#[test]
fn test_config_file() {
    let file = config_file("/tmp", "turtle");
    let expect = "/tmp/turtle.config.toml";
    assert_eq!(file, expect);
}

#[test]
fn test_compose_file() {
    let file = compose_file("/tmp", "turtle");
    let expect = "/tmp/turtle.compose.yml";
    assert_eq!(file, expect);
}

#[test]
fn test_history_file() {
    let file = history_file("/tmp", "turtle");
    let expect = "/tmp/turtle.history";
    assert_eq!(file, expect);
}

#[test]
fn test_current_directory() {
    let cur_dir = current_directory();
    assert!(!cur_dir.is_empty());
    assert!(cur_dir.ends_with("turtle"));
}

#[test]
fn test_current_directory_shortened() {
    let max_len = 32;
    let dir = current_directory_shortened(max_len);
    let len = dir.len();
    assert!(len >= 1);
    assert!(len <= max_len);
    assert!(dir.ends_with("turtle"));
}

#[test]
fn test_change_directory() {
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

#[test]
fn test_path_exist() {
    assert!(path_exist("src/config/test.rs"));
    assert!(!path_exist("unknown/path/"));
}

#[test]
fn test_normalize_spaces() {
    let text = "   this   is  a        simple    world     ";
    let expect = "this is a simple world";

    let output = normalize_spaces(text);
    assert_eq!(output, expect);
}

#[test]
fn test_replace_shortcuts() {
    let text = "git clone git@gitlab.com:phamlequang/turtle.git";
    let expect = "clone git@gitlab.com:phamlequang/turtle.git";

    let shortcut = Shortcut {
        value: String::from("clone"),
        prefixes: vec![String::from("git clone")],
    };

    let output = replace_shortcuts(text, &[shortcut]);
    assert_eq!(output, expect);
}
