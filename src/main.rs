use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::{
    parser::{flattener::flatten, parse},
    tokenizer::TokenData,
};

mod parser;
mod tokenizer;
fn main() {
    let file = File::open("test.liu").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let token_data: Vec<TokenData> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| tokenizer::tokenize(line, i))
        .collect();

    let expressions = parse(&token_data, 0);

    println!("{:#?}", expressions);

    let flattened = flatten(&expressions.0);

    println!("{:#?}", flattened);
}
