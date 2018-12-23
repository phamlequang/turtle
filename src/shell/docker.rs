use super::cmd::Command;
use crate::config::DockerMachine;

pub fn machine_command(args: Vec<String>) -> Command {
    return Command::new("docker-machine", args, "", true);
}

pub fn create_machine(machine: &DockerMachine) -> Command {
    let args = vec![
        String::from("create"),
        String::from("--driver"),
        String::from("virtualbox"),
        String::from("--virtualbox-host-dns-resolver"),
        String::from("--virtualbox-cpu-count"),
        format!("{}", machine.cpu_count),
        String::from("--virtualbox-disk-size"),
        format!("{}", machine.disk_size),
        String::from("--virtualbox-memory"),
        format!("{}", machine.memory),
        machine.name.clone(),
    ];
    return machine_command(args);
}

pub fn do_with_machine(action: &str, machine: &DockerMachine) -> Command {
    let args = vec![action.to_owned(), machine.name.clone()];
    return machine_command(args);
}

pub fn update_certificates(machine: &DockerMachine) -> Command {
    let args = vec![
        String::from("regenerate-certs"),
        String::from("--force"),
        String::from("--client-certs"),
        machine.name.clone(),
    ];
    return machine_command(args);
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

        assert_eq!(command.display(), expect);
        assert_eq!(command.program, "docker-machine");
        assert_eq!(command.args.len(), 11);
        assert!(command.dir.is_empty());
        assert!(command.verbose);
    }

    #[test]
    fn test_do_with_machine() {
        let machine = DockerMachine::default();

        let command = do_with_machine("restart", &machine);
        let expect = "docker-machine restart turtle";

        assert_eq!(command.display(), expect);
        assert_eq!(command.program, "docker-machine");
        assert_eq!(command.args.len(), 2);
        assert!(command.dir.is_empty());
        assert!(command.verbose);
    }

    #[test]
    fn test_update_certificates() {
        let machine = DockerMachine::default();

        let command = update_certificates(&machine);
        let expect = "docker-machine regenerate-certs --force --client-certs turtle";

        assert_eq!(command.display(), expect);
        assert_eq!(command.program, "docker-machine");
        assert_eq!(command.args.len(), 4);
        assert!(command.dir.is_empty());
        assert!(command.verbose);
    }
}
