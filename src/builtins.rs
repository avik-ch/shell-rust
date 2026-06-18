use std::env;

use crate::helpers::find_executable;

pub enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Self> {
        match name {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            "pwd" => Some(Builtin::Pwd),
            "cd" => Some(Builtin::Cd),
            _ => None,
        }
    }

    pub fn execute(&self, args: &[String]) -> i32 {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => {
                println!("{}", args.join(" "));
                0
            }
            Builtin::Type => Self::type_executable(&args[0]),
            Builtin::Pwd => Self::pwd_executable(),
            Builtin::Cd => {
                Self::cd_executable(&args[0]);
                0
            }
        }
    }

    fn type_executable(command: &str) -> i32 {
        match Self::lookup(command) {
            Some(_) => {
                println!("{command} is a shell builtin");
            }
            None => match find_executable(command) {
                Some(path) => {
                    println!("{} is {}", command, path.to_string_lossy());
                }
                None => {
                    println!("{command}: not found");
                }
            },
        }
        0
    }

    fn pwd_executable() -> i32 {
        if let Ok(cwd) = std::env::current_dir() {
            println!("{}", cwd.display());
            0
        } else {
            eprintln!("pwd: error retrieving current directory");
            1
        }
    }

    // TODO:: Cleanup this function
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
}
