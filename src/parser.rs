use std::{
    io::{self, Write},
    iter, mem,
    path::PathBuf,
    str,
};

use crate::command::{Command, Redirect, RedirectType, SimpleCommand};
use anyhow::{Error, Ok};

pub fn tokenise(line: &str) -> Result<Command, Error> {
    let mut command = Command::new();
    let mut cmd = SimpleCommand::new();
    let mut letters = line.trim().chars().peekable();
    let mut cur_arg = String::new();
    let mut arg_started = false;

    while let Some(&letter) = letters.peek() {
        match letter {
            '\'' | '"' => {
                arg_started = true;
                let quote = letters.next().unwrap();
                let (mut arg, closed) = parse_quote(&mut letters, quote);
                if !closed {
                    get_remaining_quote(&mut arg, quote);
                }
                cur_arg.push_str(&arg);
            }
            '\\' => {
                arg_started = true;
                letters.next();
                if let Some(&next_letter) = letters.peek() {
                    cur_arg.push(next_letter);
                    letters.next();
                } else {
                    // TODO: handle backslashes at the end of the line
                    cur_arg.push('\\');
                }
            }
            '>' => {
                letters.next();

                let redirect_type = match cur_arg.as_str() {
                    "1" | "" => {
                        arg_started = false;
                        RedirectType::StdOut
                    }
                    "2" => {
                        arg_started = false;
                        RedirectType::StdErr
                    }
                    _ => {
                        cmd.args.push(mem::take(&mut cur_arg));
                        arg_started = false;
                        RedirectType::StdOut
                    }
                };
                cur_arg.clear();
                handle_redirect(&mut letters, redirect_type, &mut cmd)?;
            }
            '|' => {
                letters.next();
                if arg_started {
                    cmd.args.push(mem::take(&mut cur_arg));
                    arg_started = false;
                }

                if cmd.args.is_empty() {
                    return Err(Error::msg("parse error near `|'"));
                }

                command.push(mem::replace(&mut cmd, SimpleCommand::new()));
            }
            c if c.is_whitespace() => {
                letters.next();
                if arg_started {
                    cmd.args.push(mem::take(&mut cur_arg));
                    arg_started = false;
                }
            }
            _ => {
                arg_started = true;
                cur_arg.push(letters.next().unwrap());
            }
        }
    }

    if arg_started {
        cmd.args.push(cur_arg);
    }

    if cmd.args.is_empty() {
        return Err(Error::msg("parse error: expected command"));
    }

    command.push(cmd);
    Ok(command)
}

fn parse_quote(letters: &mut impl Iterator<Item = char>, quote: char) -> (String, bool) {
    let mut cur_arg = String::new();
    while let Some(letter) = letters.next() {
        if quote == '\'' {
            match letter {
                '\'' => return (cur_arg, true),
                _ => cur_arg.push(letter),
            }
        } else {
            match letter {
                '\\' => {
                    if let Some(next_letter) = letters.next() {
                        match next_letter {
                            '"' => cur_arg.push('"'),
                            '\\' => cur_arg.push('\\'),
                            _ => {
                                cur_arg.push('\\');
                                cur_arg.push(next_letter);
                            }
                        }
                    }
                }
                '"' => return (cur_arg, true),
                _ => cur_arg.push(letter),
            }
        }
    }

    (cur_arg, false)
}

fn get_remaining_quote(arg: &mut String, quote: char) {
    let mut input = String::new();
    loop {
        print!("quote> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (new_arg, closed) = parse_quote(&mut input.trim().chars().peekable(), quote);
        arg.push_str(&new_arg);
        if closed {
            break;
        }
        input.clear();
    }
}

fn handle_redirect(
    letters: &mut iter::Peekable<str::Chars>,
    redirect_type: RedirectType,
    cmd: &mut SimpleCommand,
) -> Result<(), Error> {
    let append = letters.next_if(|letter| *letter == '>').is_some();

    while letters.next_if(|c| c.is_whitespace()).is_some() {}

    let (file_path, started) = parse_redirect_path(letters);
    if !started {
        return Err(Error::msg("parse error near \n"));
    }

    let redirect = Redirect {
        path: PathBuf::from(file_path),
        append,
    };

    match redirect_type {
        RedirectType::StdOut => cmd.std_out = Some(redirect),
        RedirectType::StdErr => cmd.std_err = Some(redirect),
    }

    Ok(())
}

fn parse_redirect_path(letters: &mut iter::Peekable<str::Chars>) -> (String, bool) {
    let mut path = String::new();
    let mut started = false;

    while let Some(&letter) = letters.peek() {
        match letter {
            '\'' | '"' => {
                started = true;
                let quote = letters.next().unwrap();
                let (mut value, closed) = parse_quote(letters, quote);
                if !closed {
                    get_remaining_quote(&mut value, quote);
                }
                path.push_str(&value);
            }
            '\\' => {
                started = true;
                letters.next();
                if let Some(letter) = letters.next() {
                    path.push(letter);
                } else {
                    path.push('\\');
                }
            }
            '|' | '>' => break,
            c if c.is_whitespace() => break,
            _ => {
                started = true;
                path.push(letters.next().unwrap());
            }
        }
    }

    (path, started)
}
