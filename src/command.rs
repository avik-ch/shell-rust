use std::path::PathBuf;

pub enum RedirectType {
    StdOut,
    StdErr,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Redirect {
    pub path: PathBuf,
    pub append: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleCommand {
    pub args: Vec<String>,
    pub std_out: Option<Redirect>,
    pub std_err: Option<Redirect>,
}

impl SimpleCommand {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            std_out: None,
            std_err: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    simple_commands: Vec<SimpleCommand>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            simple_commands: Vec::new(),
        }
    }

    pub fn push(&mut self, command: SimpleCommand) {
        self.simple_commands.push(command);
    }

    pub fn into_simple_commands(self) -> Vec<SimpleCommand> {
        self.simple_commands
    }
}
