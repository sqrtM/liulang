use std::{
    fs::File,
    io::{self, BufRead},
    rc::Rc,
};

use crate::{analyzer, parser};

pub fn evaluate(path: &str) -> Rc<analyzer::Node> {
    let file = File::open(path).unwrap();

    let reader = io::BufReader::new(file);

    let tokens = reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| parser::tokenize(&line.unwrap(), i))
        .collect();
    analyzer::expressionize(&tokens).unwrap()
}

pub fn show_license_notice() {
    println!("\x1b[1;30;47mliulang 流浪 - Copyright (C) 2025                            \x1b[1;0m");
    println!(
        "\x1b[30;47mThis program comes with \x1b[1;0m\x1b[1;30;47mABSOLUTELY NO WARRANTY.              \x1b[1;0m"
    );
    println!("\x1b[30;47mThis is free software, and you are welcome to redistribute it\x1b[1;0m");
    println!("\x1b[30;47munder certain conditions; see LICENSE for details.           \x1b[1;0m");
}
