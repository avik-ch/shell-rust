#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;
mod command;
mod helpers;
mod parser;
mod shell;

fn main() {
    let mut shell = shell::Shell::new();
    shell.run();
}
