use super::*;
use std::io::ErrorKind;

fn sample_config() -> Config {
    return Config::load("etc/config.toml").unwrap();
}

#[test]
fn test_load_config_ok() {
    let result = Config::load("etc/config.toml");
    assert!(result.is_ok());

    let config = result.unwrap();
    assert!(!config.project.is_empty());
    assert!(config.using.is_some());
    assert!(config.machine.is_some());
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
    let config = sample_config();
    let name = "flowers";

    let found = config.search_repository(name);
    assert!(found.is_some());

    let repository = found.unwrap();
    assert_eq!(repository.name, name);
}

#[test]
fn test_search_repository_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_repository(name);
    assert!(found.is_none());
}

#[test]
fn test_search_group_found() {
    let config = sample_config();
    let name = "all";

    let found = config.search_group(name);
    assert!(found.is_some());

    let group = found.unwrap();
    assert_eq!(group.name, name)
}

#[test]
fn test_search_group_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_group(name);
    assert!(found.is_none());
}

#[test]
fn test_using_dependencies() {
    let mut config = sample_config();

    let using_dependencies = config.using_dependencies();
    assert_eq!(using_dependencies.len(), 2);
    assert!(using_dependencies.contains("postgres"));
    assert!(using_dependencies.contains("redis"));

    config.use_groups(vec!["rep".to_owned()]);
    let using_dependencies = config.using_dependencies();
    assert!(using_dependencies.is_empty());
}

#[test]
fn test_using_repositories() {
    let mut config = sample_config();

    let using_repositories = config.using_repositories();
    assert_eq!(using_repositories.len(), 1);
    assert!(using_repositories.contains("flowers"));

    config.use_groups(vec!["dep".to_owned()]);
    let using_repositories = config.using_repositories();
    assert!(using_repositories.is_empty());
}

#[test]
fn test_match_dependencies_and_services() {
    let config = sample_config();

    let args = vec!["dep".to_owned(), "camellia".to_owned()];
    let result = config.match_dependencies_and_services(args);
    assert_eq!(result.len(), 3);

    assert!(result.contains("camellia"));
    assert!(result.contains("postgres"));
    assert!(result.contains("redis"));
}
