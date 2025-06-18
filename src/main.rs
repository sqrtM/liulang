use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::{
    parser::{parse, preparser},
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

    let preparsed = preparser::preparse(&token_data, 0);
    println!("{:#?}", preparsed.0);

    let expressions = parse(preparsed.0, Vec::new());

    //println!("{:#?}", expressions);
}
