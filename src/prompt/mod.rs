#[cfg(test)]
mod test;

use super::shell;
use super::util;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, Helper, KeyPress};
use termion::{color, color::Cyan, color::Green, color::Yellow, style};

const CURRENT_DIR_MAX_LENGTH: usize = 20;

pub struct Prompt {
    editor: Editor<PromptHelper>,
}

impl Prompt {
    pub fn new() -> Self {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .output_stream(OutputStreamType::Stdout)
            .build();

        let helper = PromptHelper {
            completer: FilenameCompleter::new(),
        };

        let mut editor = Editor::with_config(config);
        editor.set_helper(Some(helper));
        editor.bind_sequence(KeyPress::ShiftDown, Cmd::HistorySearchForward);
        editor.bind_sequence(KeyPress::ShiftUp, Cmd::HistorySearchBackward);
        return Self { editor };
    }

    pub fn load_history(&mut self, history_file: &str) {
        if let Err(err) = self.editor.load_history(history_file) {
            println!("--> cannot load history from {}: {}", history_file, err)
        }
    }

    pub fn save_history(&self, history_file: &str) {
        if let Err(err) = self.editor.save_history(history_file) {
            println!("--> cannot save history to {}: {}", history_file, err);
        }
    }

    pub fn clear_screen(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    // Prompt current directory and read a new line from stdin
    pub fn read_line(&mut self) -> String {
        let message = self.message();

        match self.editor.readline(&message) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_ref());
                return line;
            }
            Err(_) => return String::new(),
        }
    }

    pub fn message(&self) -> String {
        let dir = util::current_directory_shortened(CURRENT_DIR_MAX_LENGTH);
        let branch = shell::current_git_branch();

        let dir = format!("{}{}➜ {}", style::Bold, color::Fg(Cyan), dir);
        let prompt = format!("{} ~ {}", color::Fg(Yellow), style::Reset);

        if branch.is_empty() {
            return format!("{}{}", dir, prompt);
        }

        let branch = format!("{} {}", color::Fg(Green), branch);
        return format!("{}{}{}", dir, branch, prompt);
    }
}

struct PromptHelper {
    completer: FilenameCompleter,
}

impl Completer for PromptHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos)
    }
}

impl Hinter for PromptHelper {
    fn hint(&self, _line: &str, _pos: usize) -> Option<String> {
        None
    }
}

impl Helper for PromptHelper {}

impl Highlighter for PromptHelper {}
