use super::shell;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, Helper, KeyPress};

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

    // Prompt current directory and read a new line from stdin
    pub fn read_line(&mut self) -> String {
        let message = format!("{} ~ ", shell::current_directory());

        match self.editor.readline(&message) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_ref());
                return line;
            }
            Err(_) => return String::new(),
        }
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
