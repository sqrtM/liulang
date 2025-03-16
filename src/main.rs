use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
struct ParseErr;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Operator(Operator),
    Value(Value),
    OpenParenthesis,
    CloseParenthesis,
}

impl FromStr for Token {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_operator = s.parse::<Operator>().map(Token::Operator);
        let parse_value = s.parse::<Value>().map(Token::Value);
        let parse_parenthesis = match s {
            "(" => Ok(Token::OpenParenthesis),
            ")" => Ok(Token::CloseParenthesis),
            _ => Err(ParseErr),
        };

        parse_operator.or(parse_value).or(parse_parenthesis)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
}

impl FromStr for Operator {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
}

impl FromStr for Value {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(res) => Ok(Self::Int(res)),
            Err(_) => Err(ParseErr),
        }
    }
}

struct LineToParse<'a> {
    line: &'a str,
}

impl<'a> LineToParse<'a> {
    fn new(line: &'a str) -> Self {
        Self { line }
    }
}

fn token_should_end(left_char: char, right_char: char) -> bool {
    right_char.is_whitespace() || left_char == '(' || left_char == ')'
}

impl<'a> Iterator for LineToParse<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut chars = self.line.char_indices();
        let curr_char = chars.next()?.1;

        let next = chars
            .find(|(_, c)| token_should_end(curr_char, *c))
            .map_or(self.line.len(), |(i, _)| i);

        let (before, after) = self.line.split_at(next);
        self.line = after;

        if before.trim().is_empty() {
            self.next()
        } else {
            Some(before.trim())
        }
    }
}

fn main() {
    let file = File::open("text.liu").unwrap();

    let reader = io::BufReader::new(file);

    let tokens = reader
        .lines()
        .flat_map(|line| tokenize(&line.unwrap()))
        .collect::<Vec<Token>>();

    println!("{:#?}", tokens);
}

fn tokenize(line: &str) -> std::vec::Vec<Token> {
    LineToParse::new(line)
        .map(|raw_token| raw_token.parse::<Token>().unwrap())
        .collect::<Vec<Token>>()
}
