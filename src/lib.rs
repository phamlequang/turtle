mod cmd;
mod config;
mod docker;
mod gen;
mod git;
mod instr;
mod prompt;
mod shell;

use ctrlc;

const CONFIG_FILE: &str = "turtle.toml";
const COMPOSE_FILE: &str = "docker-compose.yml";

// Run turtle shell
pub fn run() {
    let config = config::Config::load(CONFIG_FILE).unwrap();

    let generator = gen::Generator::new(&config);
    generator.generate_docker_compose_file(COMPOSE_FILE);

    ctrlc::set_handler(|| ()).expect("error setting ctrl-c handler");

    let mut prompt = prompt::Prompt::new();
    let mut stop = false;

    while !stop {
        let line = prompt.read_line();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }
}
