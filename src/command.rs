use std::{fmt::Display, io};

pub enum RedirectType {
    StdOut,
    StdErr,
}

pub struct SimpleCommand {
    pub args: Vec<String>,
    // pub std_in: Option<PathBuf>,
    pub std_out: Box<dyn io::Write>,
    pub std_err: Box<dyn io::Write>,
}

impl SimpleCommand {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            // std_in: None,
            std_out: Box::new(io::stdout()),
            std_err: Box::new(io::stderr()),
        }
    }

    pub fn write_stdout(&mut self, text: impl Display) {
        let _ = writeln!(self.std_out, "{}", text);
    }

    pub fn write_stderr(&mut self, text: impl Display) {
        let _ = writeln!(self.std_err, "{}", text);
    }
}

// pub struct Command {
//     simple_command: Vec<SimpleCommand>,
//     // may include background work later
// }
