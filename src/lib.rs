mod cmd;
mod config;
mod decr;
mod docker;
mod gen;
mod git;
mod instr;
mod prompt;
mod shell;
mod util;

use ctrlc;

// Run turtle shell
pub fn run() {
    let project = "turtle";

    let config_dir = util::default_config_directory();
    let history_file = util::history_file(&config_dir, project);

    let mut generator = match gen::Generator::new(&config_dir, project) {
        Ok(gnrt) => gnrt,
        Err(err) => {
            println!("failed to create generator: {}", err);
            return;
        }
    };

    let mut prompt = prompt::Prompt::new();
    prompt.load_history(&history_file);
    prompt.clear_screen();

    ctrlc::set_handler(|| ()).expect("error setting ctrl-c handler");

    let mut stop = false;
    while !stop {
        let line = prompt.read_line();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }

    prompt.save_history(&history_file);
}
