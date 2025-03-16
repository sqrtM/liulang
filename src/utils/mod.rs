use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::parser::{self};

pub fn tokenize(path: &str) -> Vec<parser::TokenData> {
    let file = File::open(path).unwrap();

    let reader = io::BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| parser::tokenize(&line.unwrap(), i))
        .collect()
}
