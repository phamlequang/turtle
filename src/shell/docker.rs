use super::cmd::Command;
use crate::config::DockerMachine;

pub fn create_machine(machine: &DockerMachine) -> Command {
    let raw = format!(
        "docker-machine create \
         --driver virtualbox \
         --virtualbox-host-dns-resolver \
         --virtualbox-cpu-count {} \
         --virtualbox-disk-size {} \
         --virtualbox-memory {} \
         {}",
        machine.cpu_count, machine.disk_size, machine.memory, machine.name
    );
    return Command::new(&raw, "", true);
}

pub fn do_with_machine(action: &str, machine: &DockerMachine) -> Command {
    let raw = format!("docker-machine {} {}", action, machine.name);
    return Command::new(&raw, "", true);
}

pub fn update_certificates(machine: &DockerMachine) -> Command {
    let raw = format!(
        "docker-machine regenerate-certs --force --client-certs {}",
        machine.name
    );
    return Command::new(&raw, "", true);
}

#[cfg(test)]
mod test {
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
    fn test_do_with_machine() {
        let machine = DockerMachine::default();

        let command = do_with_machine("restart", &machine);
        let expect = "docker-machine restart turtle";

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
}
