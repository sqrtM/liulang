use crate::parser::TokenData;

use std::env;

mod analyzer;
mod interpreter;
mod parser;
mod repl;
mod utils;

fn main() {
    utils::show_license_notice();
    match env::args().collect::<Vec<String>>().get(1) {
        Some(path) => {
            let expression_tree = utils::evaluate(path);
            println!("{:?}", interpreter::execute(expression_tree));
        }
        None => loop {
            let expression_tree = repl::evaluate();
            println!("{:?}", interpreter::execute(expression_tree));
        },
    };
}
