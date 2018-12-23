use super::cmd::Command;
use crate::config::DockerMachine;

pub fn create_machine(machine: &DockerMachine) -> Command {
    let args = vec![
        String::from("create"),
        machine.name.clone(),
        String::from("--virtualbox-host-dns-resolver"),
        String::from("--virtualbox-cpu-count"),
        format!(r#""{}""#, machine.cpu_count),
        String::from("--virtualbox-disk-size"),
        format!(r#""{}""#, machine.disk_size),
        String::from("--virtualbox-memory"),
        format!(r#""{}""#, machine.memory),
    ];

    return Command::new("docker-machine", args, "", true);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_machine() {
        let machine = DockerMachine {
            name: String::from("turtle"),
            cpu_count: 2,
            disk_size: 10240,
            memory: 4096,
        };

        let command = create_machine(&machine);
        let expect = "docker-machine create turtle \
                      --virtualbox-host-dns-resolver \
                      --virtualbox-cpu-count \"2\" \
                      --virtualbox-disk-size \"10240\" \
                      --virtualbox-memory \"4096\"";
        assert_eq!(command.display(), expect);
        assert_eq!(command.program, "docker-machine");
        assert_eq!(command.args.len(), 9);
        assert!(command.dir.is_empty());
        assert!(command.verbose);
    }
}
