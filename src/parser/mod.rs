use std::str::FromStr;

pub fn tokenize(line: &str, row: usize) -> std::vec::Vec<TokenData> {
    LineToParse::new(line)
        .map(|(raw_token, token_position)| {
            TokenData::new(raw_token, row + 1, line.len() - token_position - 1)
        })
        .collect::<Vec<TokenData>>()
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenData {
    pub token: Token,
    row: usize,
    position: usize,
}

impl TokenData {
    fn new(raw_token: &str, row: usize, position: usize) -> Self {
        Self {
            token: Token::new(raw_token),
            row,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Operator(Operator),
    Value(Value),
    OpenParenthesis,
    CloseParenthesis,
    TokenizationError(String),
}

impl Token {
    fn new(raw_token: &str) -> Self {
        raw_token
            .parse::<Self>()
            .unwrap_or(Self::TokenizationError(raw_token.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseErr;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Int(i64),
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Value::Int(
            match self {
                Value::Int(s) => s,
            } + match rhs {
                Value::Int(r) => r,
            },
        )
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Value::Int(
            match self {
                Value::Int(s) => s,
            } - match rhs {
                Value::Int(r) => r,
            },
        )
    }
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

pub struct LineToParse<'a> {
    line: &'a str,
}

impl<'a> LineToParse<'a> {
    fn new(line: &'a str) -> Self {
        Self { line }
    }
}

fn token_should_end(left_char: char, right_char: char) -> bool {
    right_char.is_whitespace() || matches!(left_char, '(' | ')') || matches!(right_char, '(' | ')')
}

impl<'a> Iterator for LineToParse<'a> {
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<(&'a str, usize)> {
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
            Some((before.trim(), self.line.len()))
        }
    }
}
