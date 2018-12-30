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
const HISTORY_FILE: &str = ".history";

// Run turtle shell
pub fn run() {
    let mut config = config::Config::load(CONFIG_FILE).unwrap();

    let mut generator = gen::Generator::new(&mut config);
    generator.generate_docker_compose_file(COMPOSE_FILE);

    ctrlc::set_handler(|| ()).expect("error setting ctrl-c handler");

    let mut prompt = prompt::Prompt::new();

    prompt.load_history(HISTORY_FILE);
    prompt.clear_screen();

    let mut stop = false;
    while !stop {
        let line = prompt.read_line();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }

    prompt.save_history(HISTORY_FILE);
}
