use std::io::{self, Write};
use std::mem;

pub fn tokenise(line: &str) -> Result<Vec<String>, &'static str> {
    let mut args = Vec::new();
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
            ' ' => {
                letters.next();
                if !cur_arg.is_empty() {
                    args.push(mem::take(&mut cur_arg));
                }
            }
            _ => cur_arg.push(letters.next().unwrap()),
        }
    }

    if !cur_arg.is_empty() {
        args.push(cur_arg);
    }
    Ok(args)
}

fn parse_quote(letters: &mut impl Iterator<Item = char>, quote: char) -> (String, bool) {
    let mut cur_arg = String::new();
    while let Some(letter) = letters.next() {
        match letter {
            x if x == quote => return (cur_arg, true),
            _ => cur_arg.push(letter),
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
