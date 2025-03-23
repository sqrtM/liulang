use utils::Pipeline;

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
            let pipeline = Pipeline::new(path.into());
            pipeline.run()
        }
        None => loop {
            let expressions = repl::evaluate();
            for expression in expressions {
                println!("{:?}", interpreter::flatten(&expression));
            }
        },
    };
}
