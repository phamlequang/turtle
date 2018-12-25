use super::*;

#[test]
fn test_create_machine() {
    let machine = DockerMachine::default();

    let command = create_machine(&machine);
    let expect = "docker-machine create \
                  --driver virtualbox \
                  --virtualbox-host-dns-resolver \
                  --virtualbox-cpu-count 2 \
                  --virtualbox-disk-size 16384 \
                  --virtualbox-memory 4096 \
                  turtle";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_update_certificates() {
    let machine = DockerMachine::default();

    let command = update_certificates(&machine);
    let expect = "docker-machine regenerate-certs --force --client-certs turtle";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_machine_command() {
    let machine = DockerMachine::default();

    let command = machine_command("restart", &machine);
    let expect = "docker-machine restart turtle";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_compose_command() {
    let command = compose_command("up -d", "turtle");
    let expect = "docker-compose -p turtle up -d";

    assert_eq!(command.raw, expect);
    assert!(command.dir.is_empty());
    assert!(command.show);
}

#[test]
fn test_generate_compose_text() {
    let config = Config::default();
    let result = generate_compose_text(&config);
    let expect = fs::read_to_string("docker-compose.yml").unwrap();
    assert_eq!(result, expect);
}
