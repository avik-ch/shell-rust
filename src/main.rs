#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;
mod helpers;
mod parser;
mod shell;

fn main() {
    shell::Shell::run();
}
