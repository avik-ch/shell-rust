#[allow(unused_imports)]
use std::io::{self, Write};

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

        println!("{}: command not found", input.trim());
    }
}
