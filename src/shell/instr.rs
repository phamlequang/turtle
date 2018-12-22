use super::cmd::Command;

#[derive(Debug)]
pub struct Instruction {
    pub commands: Vec<Command>,
    pub should_terminate: bool,
}

impl Instruction {
    pub fn new(commands: Vec<Command>, should_terminate: bool) -> Self {
        return Self {
            commands,
            should_terminate,
        };
    }

    pub fn do_nothing() -> Self {
        return Self::new(Vec::new(), false);
    }

    pub fn terminate() -> Self {
        return Self::new(Vec::new(), true);
    }

    // Executes all commands sequentially
    pub fn execute(&self) {
        for cmd in &self.commands {
            cmd.execute();
        }
    }
}
