use super::*;
use std::io::ErrorKind;

#[test]
fn test_load_config_ok() {
    let result = Config::load("turtle.toml");
    assert!(result.is_ok());

    let config = result.unwrap();
    assert!(config.dependencies.is_some());
    assert!(config.repositories.is_some());
    assert!(config.groups.is_some());
}

#[test]
fn test_load_config_not_found() {
    let result = Config::load("not_found.toml");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_parse_config_invalid() {
    let toml_text = r#"
    [machine]
    name = "turtle"

    [[dependencies]]
    name = "postgres"
    version = "11.1-alpine"
    invalid key here
    "#;

    let result = Config::parse(toml_text);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
}

#[test]
fn test_search_repository_found() {
    let config = Config::default();
    let name = "flowers";

    let found = config.search_repository(name);
    assert!(found.is_some());

    let repository = found.unwrap();
    assert_eq!(repository.name, name);
}

#[test]
fn test_search_repository_not_found() {
    let config = Config::default();
    let name = "unknown";

    let found = config.search_repository(name);
    assert!(found.is_none());
}

#[test]
fn test_search_group_found() {
    let config = Config::default();
    let name = "forest";

    let found = config.search_group(name);
    assert!(found.is_some());

    let group = found.unwrap();
    assert_eq!(group.name, name)
}

#[test]
fn test_search_group_not_found() {
    let config = Config::default();
    let name = "unknown";

    let found = config.search_group(name);
    assert!(found.is_none());
}

#[test]
fn test_using_dependencies() {
    // to do
}

#[test]
fn test_using_repositories() {
    // to do
}
