pub mod brew;
pub mod cmd;
pub mod config;
pub mod decr;
pub mod dns;
pub mod docker;
pub mod gen;
pub mod git;
pub mod instr;
pub mod prompt;
pub mod shell;
pub mod util;

use ctrlc;

// Run turtle shell for a specific project
pub fn run(project: &str) {
    let config_dir = util::default_config_directory();
    let history_file = util::history_file(&config_dir, project);

    let mut generator = match gen::Generator::new(&config_dir, project) {
        Ok(gnrt) => gnrt,
        Err(err) => {
            println!("--> cannot create generator: {}", err);
            return;
        }
    };

    let mut prompt = prompt::Prompt::new();
    prompt.load_history(&history_file);
    prompt.clear_screen();

    ctrlc::set_handler(|| ()).expect("--> cannot set ctrl-c handler");

    let mut stop = false;
    while !stop {
        let line = prompt.read_line();

        let instruction = generator.generate_instruction(&line);
        shell::run_instruction(&instruction);

        stop = instruction.should_terminate;
    }

    prompt.save_history(&history_file);
}
