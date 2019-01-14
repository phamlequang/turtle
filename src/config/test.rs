use super::*;
use std::io::ErrorKind;

const CONFIG_FILE: &str = "etc/sample.config.toml";

fn sample_config() -> Config {
    return Config::load(CONFIG_FILE).expect("failed to load config file");
}

#[test]
fn test_load_config_ok() {
    let config = sample_config();

    assert!(!config.project.is_empty());
    assert!(config.using.is_some());
    assert!(config.machine.is_some());
    assert!(config.dependencies.is_some());
    assert!(config.repositories.is_some());
    assert!(config.actions.is_some());
    assert!(config.patterns.is_some());
    assert!(config.services.is_some());
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
fn test_save_config() {
    let config = sample_config();
    let expect_text = config.to_toml().expect("failed to convert to toml");

    let new_file = "etc/new.config.toml";
    let result = config.save(new_file);
    assert!(result.is_ok());

    let output_text = fs::read_to_string(new_file).expect("failed to read new file");
    assert_eq!(output_text, expect_text);

    fs::remove_file(new_file).expect("failed to remove new file");
}

#[test]
fn test_to_toml() {
    let toml_text = fs::read_to_string(CONFIG_FILE).expect("failed to read config file");
    let config = Config::parse(&toml_text).expect("failed to parse toml to config");

    let output_text = config.to_toml().expect("failed to convert config to toml");
    assert_eq!(output_text, toml_text);
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
fn test_search_action_found() {
    let config = sample_config();
    let name = "cargo";

    let found = config.search_action(name);
    assert!(found.is_some());

    let action = found.unwrap();
    assert_eq!(action.name, name);
}

#[test]
fn test_search_action_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_action(name);
    assert!(found.is_none());
}

#[test]
fn test_search_service_found() {
    let config = sample_config();
    let name = "lotus";

    let found = config.search_service(name);
    assert!(found.is_some());

    let service = found.unwrap();
    assert_eq!(service.name, name);
}

#[test]
fn test_search_service_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_service(name);
    assert!(found.is_none());
}

#[test]
fn test_search_service_repository_found() {
    let config = sample_config();
    let name = "lotus";

    let found = config.search_service_repository(name);
    assert!(found.is_some());

    let repository = found.unwrap();
    assert_eq!(repository.name, "flowers");
}

#[test]
fn test_search_service_repository_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_service_repository(name);
    assert!(found.is_none());
}

#[test]
fn test_search_service_directory_found() {
    let config = sample_config();
    let name = "camellia";

    let found = config.search_service_directory(name);
    assert!(found.is_some());

    let dir = found.unwrap();
    assert_eq!(dir, "~/projects/flowers/camellia");
}

#[test]
fn test_search_service_directory_not_found() {
    let config = sample_config();
    let name = "unknown";

    let found = config.search_service_directory(name);
    assert!(found.is_none());
}

#[test]
fn test_service_directory() {
    let config = sample_config();
    let name = "camellia";

    let service = config.search_service(name);
    assert!(service.is_some());
    let service = service.unwrap();

    let found = config.service_directory(&service);
    assert!(found.is_some());

    let dir = found.unwrap();
    assert_eq!(dir, "~/projects/flowers/camellia");
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

    let dependencies = config.using_dependencies();
    assert_eq!(dependencies.len(), 2);
    assert!(dependencies.contains("postgres"));
    assert!(dependencies.contains("redis"));

    config.use_groups(&["svc"]);
    let dependencies = config.using_dependencies();
    assert!(dependencies.is_empty());
}

#[test]
fn test_using_services() {
    let mut config = sample_config();

    let services = config.using_services();
    assert_eq!(services.len(), 2);
    assert!(services.contains("camellia"));
    assert!(services.contains("lotus"));

    config.use_groups(&["dep"]);
    let services = config.using_services();
    assert!(services.is_empty());
}

#[test]
fn test_using_repositories() {
    let mut config = sample_config();

    let repositories = config.using_repositories();
    assert_eq!(repositories.len(), 1);
    assert!(repositories.contains("flowers"));

    config.use_groups(&["dep"]);
    let repositories = config.using_repositories();
    assert!(repositories.is_empty());
}

#[test]
fn test_match_services_dependencies() {
    let config = sample_config();

    let args = ["dep", "camellia", "unknown"];
    let result = config.match_services_dependencies(&args, Config::BOTH);
    assert_eq!(result.len(), 3);

    assert!(result.contains("camellia"));
    assert!(result.contains("postgres"));
    assert!(result.contains("redis"));
}

#[test]
fn test_match_services_dependencies_repo() {
    let config = sample_config();

    let args = ["flowers", "unknown"];
    let result = config.match_services_dependencies(&args, Config::BOTH);
    assert_eq!(result.len(), 2);

    assert!(result.contains("camellia"));
    assert!(result.contains("lotus"));
}

#[test]
fn test_match_services_dependencies_all() {
    let config = sample_config();

    let result = config.match_services_dependencies(&[], Config::BOTH);
    assert_eq!(result.len(), 4);

    assert!(result.contains("camellia"));
    assert!(result.contains("lotus"));
    assert!(result.contains("postgres"));
    assert!(result.contains("redis"));
}

#[test]
fn test_match_services_only() {
    let config = sample_config();

    let result = config.match_services_dependencies(&[], Config::SERVICE);
    assert_eq!(result.len(), 2);

    assert!(result.contains("camellia"));
    assert!(result.contains("lotus"));
}

#[test]
fn test_match_dependencies_only() {
    let config = sample_config();

    let result = config.match_services_dependencies(&["all"], Config::DEPENDENCY);
    assert_eq!(result.len(), 2);

    assert!(result.contains("postgres"));
    assert!(result.contains("redis"));
}

#[test]
fn test_fill_patterns() {
    let config = sample_config();
    let service = config.search_service("camellia").unwrap();

    let filled_text = config.fill_patterns("{REPO_DIR}:/app", None);
    let expect_text = "{REPO_DIR}:/app";
    assert_eq!(filled_text, expect_text);

    let filled_text = config.fill_patterns("{REPO_DIR}:/app", Some(&service));
    let expect_text = "~/projects/flowers:/app";
    assert_eq!(filled_text, expect_text);

    let filled_text = config.fill_patterns("{SERVICE_DIR}/.env", Some(&service));
    let expect_text = "~/projects/flowers/camellia/.env";
    assert_eq!(filled_text, expect_text);
}
