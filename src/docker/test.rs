use super::*;

fn sample_machine() -> Machine {
    return Machine {
        name: String::from("turtle"),
        cpu_count: 2,
        disk_size: 16384,
        memory: 4096,
        volumes: None,
    };
}

#[test]
fn test_create_machine() {
    let machine = sample_machine();

    let command = create_machine(&machine);
    let expect = Command::basic_show(
        "docker-machine create \
         --driver virtualbox \
         --virtualbox-host-dns-resolver \
         --virtualbox-cpu-count 2 \
         --virtualbox-disk-size 16384 \
         --virtualbox-memory 4096 \
         turtle",
    );

    assert_eq!(command, expect);
}

#[test]
fn test_update_certificates() {
    let machine = sample_machine();

    let command = update_certificates(&machine);
    let expect = Command::basic_show(
        "docker-machine regenerate-certs \
         --force --client-certs turtle",
    );

    assert_eq!(command, expect);
}

#[test]
fn test_load_environments() {
    let machine = sample_machine();

    let command = load_environments(&machine);
    let expect = Command::basic_show("eval \"$(docker-machine env turtle)\"");
    assert_eq!(command, expect);
}

#[test]
fn test_machine_command() {
    let machine = sample_machine();

    let command = machine_command("restart", &machine);
    let expect = Command::basic_show("docker-machine restart turtle");

    assert_eq!(command, expect);
}

#[test]
fn test_compose_command() {
    let command = compose_command("up -d", "forest", "compose.yml");
    let expect = Command::basic_show("docker-compose -p forest -f compose.yml up -d");
    assert_eq!(command, expect);
}

#[test]
fn test_service_logs() {
    let command = service_logs("lotus", "forest", "compose.yml");
    let expect = Command::basic_show(
        "docker-compose -p forest -f compose.yml \
         logs -f --tail=100 lotus",
    );
    assert_eq!(command, expect);
}

#[test]
fn test_restart_services() {
    let service_names = vec!["lotus".to_owned(), "camellia".to_owned()];
    let command = restart_services(service_names, "forest", "compose.yml");
    let expect = Command::basic_show(
        "docker-compose -p forest -f compose.yml \
         restart lotus camellia",
    );
    assert_eq!(command, expect);
}

#[test]
fn test_docker_command() {
    let command = docker_command("images");
    let expect = Command::basic_hide("docker images");
    assert_eq!(command, expect);
}

#[test]
fn test_list_containers() {
    let command = list_containers();
    let expect = Command::basic_hide(
        "docker ps -a --format \"table \
         {{.Names}}\t{{.Image}}\t{{.Size}}\t{{.CreatedAt}}\t{{.Status}}\"",
    );
    assert_eq!(command, expect);
}

#[test]
fn test_generate_compose_text() {
    let config = Config::load("etc/config.toml").unwrap();
    let result = generate_compose_text(&config);
    let expect = fs::read_to_string("etc/compose.yml").unwrap();
    assert_eq!(result, expect);
}
