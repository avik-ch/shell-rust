#[allow(unused_imports)]
use std::{
    env, fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
};

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
            "type" => println!("{} ", type_exec(args)),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn type_exec(arg: &str) -> String {
    if KNOWN_COMMANDS.contains(&arg) {
        return format!("{arg} is a shell builtin");
    }

    let paths = env::var("PATH").unwrap_or_else(|err| {
        eprintln!("Issue with env var PATH {}", err);
        return String::new();
    });

    for dir in env::split_paths(&paths) {
        let full_path = Path::new(&dir).join(arg);

        if full_path.exists() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 != 0 {
                    return format!("{} is {}", arg, full_path.to_string_lossy());
                }
            }
        }
    }

    format!("{arg}: not found")
}
