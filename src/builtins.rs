use crate::{command::SimpleCommand, helpers::find_executable};
use std::env;

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
            "Cd" => Some(Builtin::Cd),
            _ => None,
        }
    }

    pub fn execute(&self, command: &mut SimpleCommand) {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => {
                command.write_stdout(command.args.join(" ").as_str());
            }
            Builtin::Type => Self::type_executable(command),
            Builtin::Pwd => Self::pwd_executable(command),
            Builtin::Cd => {
                Self::cd_executable(command);
            }
        }
    }

    fn type_executable(command: &mut SimpleCommand) {
        let exec = &command.args[0];
        match Self::lookup(exec) {
            Some(_) => {
                command.write_stdout(format!("{exec} is a shell builtin").as_str());
            }
            None => match find_executable(exec) {
                Some(path) => {
                    command
                        .write_stdout(format!("{} is {}", exec, path.to_string_lossy()).as_str());
                }
                None => {
                    command.write_stderr(format!("{exec}: not found").as_str());
                }
            },
        }
    }

    fn pwd_executable(command: &mut SimpleCommand) {
        if let Ok(cwd) = std::env::current_dir() {
            command.write_stdout(format!("{}", cwd.display()).as_str());
        } else {
            command.write_stderr(format!("pwd: error retrieving current directory").as_str());
        }
    }

    // TODO:: Cleanup this function
    fn cd_executable(command: &mut SimpleCommand) {
        let path = &command.args[0];

        let res = match path.as_str() {
            "~" => env::set_current_dir(env::home_dir().unwrap()),
            _ => env::set_current_dir(path),
        };

        match res {
            Err(_) => {
                command.write_stderr(format!("cd: {}: No such file or directory", path).as_str())
            }
            _ => {}
        }
    }
}
