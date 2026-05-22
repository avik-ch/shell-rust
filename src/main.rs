#[allow(unused_imports)]
use std::{
    env, fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

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
            "type" => println!("{}", type_exec(args)),
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            "cd" => cd_executable(args),
            _ => {
                let Some(_) = find_executable(command) else {
                    println!("{}: command not found", input.trim());
                    continue;
                };

                let _ = Command::new(command)
                    .args(args.split(" "))
                    .spawn()
                    .expect("Failed to execute process")
                    .wait();
            }
        }
    }
}

fn type_exec(arg: &str) -> String {
    if BUILTIN_COMMANDS.contains(&arg) {
        return format!("{arg} is a shell builtin");
    }

    match find_executable(arg) {
        Some(path) => format!("{} is {}", arg, path.to_string_lossy()),
        None => format!("{arg}: not found"),
    }
}

fn find_executable(executable: &str) -> Option<PathBuf> {
    let Ok(paths) = env::var("PATH") else {
        return None;
    };

    for dir in env::split_paths(&paths) {
        let full_path = Path::new(&dir).join(executable);

        if full_path.exists() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 != 0 {
                    return Some(full_path);
                }
            }
        }
    }

    None
}

fn cd_executable(arg: &str) {
    match arg {
        "~" => env::set_current_dir(
            env::home_dir().expect("Something went wrong reading the HOME env var"),
        )
        .unwrap_or_else(|_| println!("Error reading HOME env var")),
        _ => env::set_current_dir(arg)
            .unwrap_or_else(|_| println!("cd: {}: No such file or directory", arg)),
    }
}
