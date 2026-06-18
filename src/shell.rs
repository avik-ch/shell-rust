use std::{
    io::{self, Write},
    process::Command,
};

use crate::builtins::Builtin;
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

            let mut args = tokenise(&input).unwrap_or_else(|_| {
                eprintln!("Something wrong with parsing the last command");
                vec!["".to_string()]
            });

            let command = args.remove(0);

            // let (command, args) = input
            //     .trim()
            //     .split_once(' ')
            //     .unwrap_or_else(|| (input.trim(), ""));
            // let args: Vec<&str> = args.split_whitespace().collect();

            if let Some(cmd) = Builtin::lookup(&command) {
                Builtin::execute(&cmd, &args);
            } else {
                let Some(_) = find_executable(&command) else {
                    println!("{}: command not found", input.trim());
                    continue;
                };

                let _ = Command::new(command)
                    .args(args)
                    .spawn()
                    .expect("Failed to execute process")
                    .wait();
            }
            input.clear();
        }
    }
}
