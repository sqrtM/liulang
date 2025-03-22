use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    rc::Rc,
};

use crate::{
    analyzer::{self, Node},
    parser::{self, TokenData},
};

enum InputType {
    Repl,
    File(PathBuf),
}

enum ContextType {
    Input(InputContext),
    Parser(ParserContext),
    Analyzer(AnalyzerContext),
}

struct InputContext {
    input_type: InputType,
    line: String,
    line_number: usize,
}

impl InputContext {
    fn new(input_type: InputType, line: String, line_number: usize) -> Self {
        Self {
            input_type,
            line,
            line_number,
        }
    }
}

struct ParserContext {
    tokens: Vec<TokenData>,
    input_context: InputContext,
}

impl ParserContext {
    fn new(tokens: Vec<TokenData>, input_context: InputContext) -> Self {
        Self {
            tokens,
            input_context,
        }
    }
}

struct AnalyzerContext;

impl AnalyzerContext {
    fn new() -> Self {
        Self
    }
}

struct Pipeline;

impl Pipeline {
    fn new(ctx: ContextType) -> Self {
        Self
    }
}

pub fn evaluate(path: &str) -> Vec<Rc<Node>> {
    let file = File::open(path).unwrap();

    let reader = io::BufReader::new(file);

    let tokens = reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| parser::tokenize(&line.unwrap(), i))
        .collect();
    analyzer::expressionize(&tokens).0
}

pub fn show_license_notice() {
    println!("\x1b[1;30;47mliulang 流浪 - Copyright (C) 2025                            \x1b[1;0m");
    println!(
        "\x1b[30;47mThis program comes with \x1b[1;0m\x1b[1;30;47mABSOLUTELY NO WARRANTY.              \x1b[1;0m"
    );
    println!("\x1b[30;47mThis is free software, and you are welcome to redistribute it\x1b[1;0m");
    println!("\x1b[30;47munder certain conditions; see LICENSE for details.           \x1b[1;0m");
}
