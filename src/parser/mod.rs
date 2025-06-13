use std::{rc::Rc, str::FromStr};

pub fn tokenize(line: &str, row: usize) -> std::vec::Vec<TokenData> {
    LineToParse::new(line)
        .map(|(raw_token, token_position)| {
            TokenData::new(
                raw_token,
                row + 1,
                line.chars().count() - token_position - 1,
            )
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
    Value(Value),
    OpenParenthesis,
    CloseParenthesis,
    TokenizationError(String),
    Unit,
}

impl Default for Token {
    fn default() -> Self {
        Self::Unit
    }
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
        let tokenize_value = s.parse::<Value>().map(Token::Value);
        let tokenize_unit = if s == "()" {
            Ok(Token::Unit)
        } else {
            Err(ParseErr)
        };
        let tokenize_parenthesis = match s {
            "(" => Ok(Token::OpenParenthesis),
            ")" => Ok(Token::CloseParenthesis),
            _ => Err(ParseErr),
        };

        tokenize_parenthesis.or(tokenize_unit).or(tokenize_value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Int(i64),
    Identifier(Rc<String>),
    Unit,
    // Later, we can have Strings and other stuff.
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Value::Int(
            match self {
                Value::Int(s) => s,
                _ => todo!(),
            } + match rhs {
                Value::Int(r) => r,
                _ => todo!("{:?}", rhs),
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
                _ => todo!(),
            } - match rhs {
                Value::Int(r) => r,
                _ => todo!(),
            },
        )
    }
}

impl FromStr for Value {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(res) => Ok(Self::Int(res)),
            Err(_) => Ok(Self::Identifier(Rc::new(s.to_string()))),
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

impl<'a> Iterator for LineToParse<'a> {
    // The usize here, used to get the location of the problem char in case of error,
    // is the DISTANCE FROM THE END OF THE LINE. So some calculations have to be made
    // after the fact in order to get its real "location".
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.line.char_indices();
        let (_, curr_char) = chars.next()?;

        let next = chars.find(|(_, c)| token_should_end(curr_char, *c)).map_or(
            self.line.len(),
            |(i, next_char)| {
                if curr_char == '(' && next_char == ')' {
                    i + 1
                } else {
                    i
                }
            },
        );

        let (token, rest) = self.line.split_at(next);

        self.line = rest;

        if token.trim().is_empty() {
            self.next()
        } else {
            Some((token.trim(), self.line.len()))
        }
    }
}

fn token_should_end(left_char: char, right_char: char) -> bool {
    right_char.is_whitespace() || matches!(left_char, '(' | ')') || matches!(right_char, '(' | ')')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_token() {
        let line = "token";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), Some(("token", 0)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_multiple_tokens() {
        let line = "token1 token2 token3";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), Some(("token1", 14)));
        assert_eq!(parser.next(), Some(("token2", 7)));
        assert_eq!(parser.next(), Some(("token3", 0)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_leading_and_trailing_whitespace() {
        let line = "   token1   token2   ";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), Some(("token1", 12)));
        assert_eq!(parser.next(), Some(("token2", 3)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_unit_token() {
        let line = "def () token";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), Some(("def", 9)));
        assert_eq!(parser.next(), Some(("()", 6)));
        assert_eq!(parser.next(), Some(("token", 0)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_complex_unit_tokens() {
        let line = "(def a () (+ 2 1))";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), Some(("(", 17)));
        assert_eq!(parser.next(), Some(("def", 14)));
        assert_eq!(parser.next(), Some(("a", 12)));
        assert_eq!(parser.next(), Some(("()", 9)));
        assert_eq!(parser.next(), Some(("(", 7)));
        assert_eq!(parser.next(), Some(("+", 6)));
        assert_eq!(parser.next(), Some(("2", 4)));
        assert_eq!(parser.next(), Some(("1", 2)));
        assert_eq!(parser.next(), Some((")", 1)));
        assert_eq!(parser.next(), Some((")", 0)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_only_whitespace() {
        let line = "   ";
        let mut parser = LineToParse::new(line);

        assert_eq!(parser.next(), None);
    }
}
