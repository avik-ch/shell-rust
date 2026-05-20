#[allow(unused_imports)]
use std::io::{self, Write};

const KNOWN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn main() {
    // stage 3
    loop {
        // stage 1
        print!("$ ");
        io::stdout().flush().unwrap();

        // stage 2
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (command, args) = input
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| (input.trim(), ""));

        match command {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => get_type(args),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn get_type(arg: &str) {
    if KNOWN_COMMANDS.contains(&arg) {
        println!("{arg} is a shell builtin");
    } else {
        println!("{arg}: not found");
    }
}
