use std::{
    io::{self, Write},
    process::Command,
};

use crate::builtins::Builtin;
use crate::command::SimpleCommand;
use crate::helpers::find_executable;
use crate::parser::tokenise;

pub struct Shell;

impl Shell {
    pub fn run() {
        let mut input = String::new();
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim().is_empty() {
                continue;
            }

            let mut cmd = tokenise(&input).unwrap_or_else(|e| {
                eprintln!("{}", e);
                SimpleCommand::new()
            });

            let executable = cmd.args.remove(0);
            let executable = executable.trim();

            if let Some(exec) = Builtin::lookup(&executable) {
                Builtin::execute(&exec, &mut cmd);
            } else {
                let Some(_) = find_executable(&executable) else {
                    // TODO: move this logic to builtins completely
                    cmd.write_stderr(format!("{}: command not found", input.trim()).as_str());
                    input.clear();
                    continue;
                };

                match Command::new(executable).args(&cmd.args).output() {
                    Ok(output) => {
                        let _ = cmd.std_out.write_all(&output.stdout);
                        let _ = cmd.std_err.write_all(&output.stderr);
                    }
                    Err(e) => cmd.write_stderr(e),
                }
                // .stdout(std_out)
                // .stderr(std_err)
                // .spawn()
                // .expect("Failed to execute process")
                // .wait();
            }

            input.clear();
        }
    }
}
