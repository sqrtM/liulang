use std::{
    cell::RefCell,
    fmt,
    fs::File,
    io::{self, BufRead},
    rc::Rc,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
struct TokenData {
    token: Token,
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
enum Token {
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
struct ParseErr;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Value {
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

struct LineToParse<'a> {
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

fn main() {
    let file = File::open("text.liu").unwrap();

    let reader = io::BufReader::new(file);

    let tokens = reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| tokenize(&line.unwrap(), i))
        .collect::<Vec<TokenData>>();

    let tree = expressionize(tokens);
    println!("{:?}", execute(tree));
}

fn tokenize(line: &str, row: usize) -> std::vec::Vec<TokenData> {
    LineToParse::new(line)
        .map(|(raw_token, token_position)| {
            TokenData::new(raw_token, row + 1, line.len() - token_position - 1)
        })
        .collect::<Vec<TokenData>>()
}

#[derive(PartialEq, Eq)]
struct Node {
    token: Token,
    parent: Option<Rc<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.clone()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = self.parent.as_ref().map(|p| p.clone());

        let c = self.children.borrow();
        let childs = c
            .iter()
            .map(|child| child.token.clone())
            .collect::<Vec<Token>>();

        write!(
            f,
            "Node {{ token: {:?}, parent: {:?}, children: {:?} }}",
            self.token, p, childs
        )
    }
}

fn expressionize(tokens: std::vec::Vec<TokenData>) -> Rc<Node> {
    let mut current_node: Option<Rc<Node>> = None;
    let mut depth = 0;

    for token_data in tokens {
        match token_data.token {
            Token::Operator(operator) => {
                let new_node = Rc::new(Node {
                    token: Token::Operator(operator),
                    parent: current_node.clone(),
                    children: RefCell::new(vec![]),
                });

                if let Some(ref c_n) = current_node {
                    c_n.children.borrow_mut().push(new_node.clone());
                    current_node = Some(new_node);
                } else {
                    current_node = Some(new_node);
                }
            }
            Token::Value(value) => {
                let new_node = Rc::new(Node {
                    token: Token::Value(value),
                    parent: current_node.clone(),
                    children: RefCell::new(vec![]),
                });

                if let Some(ref c_n) = current_node {
                    c_n.children.borrow_mut().push(new_node.clone());
                }
            }
            Token::OpenParenthesis => depth += 1,
            Token::CloseParenthesis => {
                depth -= 1;
                if depth > 0 {
                    current_node = current_node.unwrap().get_parent()
                }
            }
            Token::TokenizationError(_) => todo!(),
        }
    }

    current_node.expect("Why")
}

fn execute(t: Rc<Node>) -> Value {
    let children = t.children.borrow();
    match t.token {
        Token::Operator(operator) => match operator {
            Operator::Plus => children
                .iter()
                .fold(Value::Int(0), |acc, child| acc + execute(child.clone())),
            Operator::Minus => children
                .iter()
                .skip(1)
                .fold(execute(children[0].clone()), |acc, child| {
                    acc - execute(child.clone())
                }),
        },
        Token::Value(value) => value,
        _ => todo!(),
    }
}
