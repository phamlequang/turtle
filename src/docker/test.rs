use super::*;

#[test]
fn test_create_machine() {
    let machine = DockerMachine::default();

    let command = create_machine(&machine);
    let expect = Command::basic(
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
    let machine = DockerMachine::default();

    let command = update_certificates(&machine);
    let expect = Command::basic("docker-machine regenerate-certs --force --client-certs turtle");

    assert_eq!(command, expect);
}

#[test]
fn test_load_environments() {
    let machine = DockerMachine::default();

    let command = load_environments(&machine);
    let expect = Command::basic("eval \"$(docker-machine env turtle)\"");
    assert_eq!(command, expect);
}

#[test]
fn test_machine_command() {
    let machine = DockerMachine::default();

    let command = machine_command("restart", &machine);
    let expect = Command::basic("docker-machine restart turtle");

    assert_eq!(command, expect);
}

#[test]
fn test_compose_command() {
    let command = compose_command("up -d", "turtle");
    let expect = Command::basic("docker-compose -p turtle up -d");
    assert_eq!(command, expect);
}

#[test]
fn test_service_logs() {
    let command = service_logs("lotus", "turtle");
    let expect = Command::basic("docker-compose -p turtle logs --tail=100 lotus");
    assert_eq!(command, expect);
}

#[test]
fn test_docker_command() {
    let command = docker_command("images");
    let expect = Command::basic("docker images");
    assert_eq!(command, expect);
}

#[test]
fn test_list_containers() {
    let command = list_containers();
    let expect = Command::basic(
        "docker ps -a --format \"table \
         {{.Names}}\t{{.Image}}\t{{.Size}}\t{{.CreatedAt}}\t{{.Status}}\"",
    );
    assert_eq!(command, expect);
}

#[test]
fn test_generate_compose_text() {
    let config = Config::default();
    let result = generate_compose_text(&config);
    let expect = fs::read_to_string("docker-compose.yml").unwrap();
    assert_eq!(result, expect);
}
