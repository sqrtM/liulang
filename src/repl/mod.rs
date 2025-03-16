use std::io::{self, Write};

use crate::parser;

pub fn tokenize() -> Vec<parser::TokenData> {
    let mut input = String::new();

    print!("\x1b[1;34mliulang 流浪 v.0.0.1 🦀 v1.85.0\x1b[1;0m\n> ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    parser::tokenize(input.trim(), 0)
}
