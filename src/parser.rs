use std::{
    fs::OpenOptions,
    io::{self, Write},
    iter, mem,
    path::PathBuf,
    str,
};

use crate::command::RedirectType;
use crate::command::SimpleCommand;
use anyhow::{Error, Ok};

pub fn tokenise(line: &str) -> Result<SimpleCommand, Error> {
    let mut cmd = SimpleCommand::new();
    let mut letters = line.trim().chars().peekable();
    let mut cur_arg = String::new();

    while let Some(&letter) = letters.peek() {
        match letter {
            '\'' | '"' => {
                let quote = letters.next().unwrap();
                let (mut arg, closed) = parse_quote(&mut letters, quote);
                if !closed {
                    get_remaining_quote(&mut arg, quote);
                }
                cur_arg.push_str(&arg);
            }
            '\\' => {
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

                match cur_arg.as_str() {
                    "1" | "" => {
                        handle_redirect(&mut letters, RedirectType::StdOut, &mut cmd)?;
                    }
                    "2" => {
                        handle_redirect(&mut letters, RedirectType::StdErr, &mut cmd)?;
                    }
                    _ => {
                        cmd.args.push(mem::take(&mut cur_arg));
                        handle_redirect(&mut letters, RedirectType::StdOut, &mut cmd)?;
                    }
                }
                cur_arg.clear();
            }
            ' ' => {
                letters.next();
                if !cur_arg.is_empty() {
                    cmd.args.push(mem::take(&mut cur_arg));
                }
            }
            _ => cur_arg.push(letters.next().unwrap()),
        }
    }

    if !cur_arg.is_empty() {
        cmd.args.push(cur_arg);
    }

    Ok(cmd)
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
    let mut append = false;
    if let Some(&letter) = letters.peek() {
        match letter {
            '>' => {
                letters.next();
                append = true;
            }
            _ => {
                // error will get handled by non existent file path
            }
        }
    }

    let file_path = letters
        .by_ref()
        .skip_while(|c| c.is_whitespace())
        .collect::<String>();
    if file_path.is_empty() {
        return Err(Error::msg("parse error near \n"));
    }

    let file_path = PathBuf::from(file_path);
    let file = Box::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(!append)
            .append(append)
            .open(file_path)?,
    );

    match redirect_type {
        RedirectType::StdOut => cmd.std_out = file,
        RedirectType::StdErr => cmd.std_err = file,
    }

    Ok(())
}
