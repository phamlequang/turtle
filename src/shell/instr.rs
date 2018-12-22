use super::cmd::Command;

#[derive(Debug)]
pub struct Instruction {
    pub commands: Option<Vec<Command>>,
    pub should_terminate: bool,
}

impl Instruction {
    pub fn new(commands: Option<Vec<Command>>, should_terminate: bool) -> Self {
        return Self {
            commands,
            should_terminate,
        };
    }

    pub fn do_nothing() -> Self {
        return Self::new(None, false);
    }

    pub fn terminate() -> Self {
        return Self::new(None, true);
    }

    pub fn normal(commands: Vec<Command>) -> Self {
        return Self::new(Some(commands), false);
    }

    // Executes all commands sequentially
    pub fn execute(&self) {
        if let Some(commands) = self.commands.as_ref() {
            for cmd in commands {
                cmd.execute();
            }
        }
    }
}
