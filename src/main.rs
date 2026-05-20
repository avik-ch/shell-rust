#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let mut input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    print!("{}: command not found", input.trim());
    io::stdout().flush().unwrap();
}
