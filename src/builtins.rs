use crate::helpers::find_executable;
use std::{env, io::Write};

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

    pub fn execute(&self, args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => {
                let _ = writeln!(stdout, "{}", args.join(" "));
            }
            Builtin::Type => Self::type_executable(args, stdout, stderr),
            Builtin::Pwd => Self::pwd_executable(stdout, stderr),
            Builtin::Cd => {
                Self::cd_executable(args, stderr);
            }
        }
    }

    fn type_executable(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) {
        let Some(exec) = args.first() else {
            let _ = writeln!(stderr, "type: missing operand");
            return;
        };

        match Self::lookup(exec) {
            Some(_) => {
                let _ = writeln!(stdout, "{exec} is a shell builtin");
            }
            None => match find_executable(exec) {
                Some(path) => {
                    let _ = writeln!(stdout, "{} is {}", exec, path.to_string_lossy());
                }
                None => {
                    let _ = writeln!(stderr, "{exec}: not found");
                }
            },
        }
    }

    fn pwd_executable(stdout: &mut dyn Write, stderr: &mut dyn Write) {
        if let Ok(cwd) = std::env::current_dir() {
            let _ = writeln!(stdout, "{}", cwd.display());
        } else {
            let _ = writeln!(stderr, "pwd: error retrieving current directory");
        }
    }

    // TODO:: Cleanup this function
    fn cd_executable(args: &[String], stderr: &mut dyn Write) {
        let Some(path) = args.first() else {
            let _ = writeln!(stderr, "cd: missing operand");
            return;
        };

        let res = match path.as_str() {
            "~" => env::set_current_dir(env::home_dir().unwrap()),
            _ => env::set_current_dir(path),
        };

        if res.is_err() {
            let _ = writeln!(stderr, "cd: {}: No such file or directory", path);
        }
    }
}
