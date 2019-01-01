#[cfg(test)]
mod test;

mod cmd;
mod config;
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
    let config_dir = util::config_directory();
    let history_file = util::history_file(&config_dir);
    let mut generator = gen::Generator::new(&config_dir);

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
