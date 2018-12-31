mod cmd;
mod config;
mod docker;
mod gen;
mod git;
mod instr;
mod prompt;
mod shell;

use ctrlc;

// Run turtle shell
pub fn run() {
    let config_directory = shell::config_directory();
    let config_file = format!("{}/turtle.toml", config_directory);
    let compose_file = format!("{}/docker-compose.yml", config_directory);
    let history_file = format!("{}/history.txt", config_directory);

    let mut config: config::Config;
    match config::Config::load(&config_file) {
        Ok(cfg) => config = cfg,
        Err(err) => {
            println!("-> cannot load config file {}: {}", config_file, err);
            return;
        }
    }

    let mut generator = gen::Generator::new(&mut config);
    generator.generate_docker_compose_file(&compose_file);

    ctrlc::set_handler(|| ()).expect("error setting ctrl-c handler");

    let mut prompt = prompt::Prompt::new();

    prompt.load_history(&history_file);
    prompt.clear_screen();

    let mut stop = false;
    while !stop {
        let line = prompt.read_line();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }

    prompt.save_history(&history_file);
}
