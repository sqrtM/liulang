use crate::parser::TokenData;

use std::env;

mod analyzer;
mod interpreter;
mod parser;
mod repl;
mod utils;

fn main() {
    match env::args().collect::<Vec<String>>().get(1) {
        Some(path) => {
            let tokens = utils::tokenize(path);
            let tree = analyzer::expressionize(tokens);
            println!("{:?}", interpreter::execute(tree));
        }
        None => loop {
            let tokens = repl::tokenize();
            let tree = analyzer::expressionize(tokens);
            println!("{:?}", interpreter::execute(tree));
        },
    };
}
