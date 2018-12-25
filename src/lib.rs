mod cmd;
mod config;
mod docker;
mod gen;
mod git;
mod instr;
mod shell;

const CONFIG_FILE: &str = "turtle.toml";
const COMPOSE_FILE: &str = "docker-compose.yml";

// Run turtle shell
pub fn run() {
    let config = config::Config::load(CONFIG_FILE).unwrap();

    let generator = gen::Generator::new(&config);
    generator.generate_docker_compose_file(COMPOSE_FILE);

    let mut stop = false;

    while !stop {
        let line = shell::prompt();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }
}
