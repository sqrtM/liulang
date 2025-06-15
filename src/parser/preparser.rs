use crate::{
    parse,
    parser::{Node, Operand},
    tokenizer::TokenData,
};

use crate::tokenizer::{Token, Value};

#[derive(Debug)]
enum ValueList {
    Empty,
    Value(Value),
    List(Vec<Value>),
}

impl ValueList {
    fn push(&self, item: Value) -> ValueList {
        match self {
            ValueList::Empty => ValueList::Value(item),
            ValueList::Value(value) => ValueList::List(vec![value.clone(), item]),
            ValueList::List(value_lists) => {
                let mut val = value_lists.clone();
                val.push(item);
                ValueList::List(val)
            }
        }
    }

    fn append(&self, item: ValueList) -> ValueList {
        match self {
            ValueList::Empty => item,
            ValueList::Value(value) => {
                let mut v = vec![value.clone()];
                v.extend(item.unravel());
                ValueList::List(v)
            }
            ValueList::List(value_lists) => {
                let mut val = value_lists.clone();
                println!("{:?}", val);
                val.extend(item.unravel());
                println!("{:?}", val);
                ValueList::List(val)
            }
        }
    }

    fn unravel(&self) -> Vec<Value> {
        match self {
            ValueList::Empty => Vec::new(),
            ValueList::Value(value) => vec![value.clone()],
            ValueList::List(value_lists) => value_lists.clone(),
        }
    }
}

fn preparse(tokens: &[TokenData], mut idx: usize) -> (ValueList, usize) {
    let mut values: ValueList = ValueList::Empty;
    let mut current_list: ValueList = ValueList::Empty;

    while idx < tokens.len() {
        println!("{:?}", &tokens[idx].token);
        println!("{:?}", current_list);
        println!("{:?}\n=============", values);
        match &tokens[idx].token {
            Token::Value(value) => {
                current_list = current_list.push(value.clone());
            }
            Token::OpenParenthesis => {
                idx += 1;
                let inner_list = preparse(tokens, idx);
                current_list = current_list.append(inner_list.0);
                idx = inner_list.1;
                continue;
            }
            Token::CloseParenthesis => {
                values = values.append(current_list);
                idx += 1;
                return (values, idx);
            }
            Token::TokenizationError(_) => todo!(),
        };
        idx += 1;
    }
    values = values.append(current_list);
    (values, idx)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_parsing() {
        let tokens = vec![
            TokenData::new("(", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("1", 0, 0),
            TokenData::new("2", 0, 0),
            TokenData::new("a", 0, 0),
            TokenData::new("b", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("3", 0, 0),
            TokenData::new("4", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (node, next_idx) = preparse(&tokens, 0);

        panic!("{:?}", node)

        // assert_eq!(next_idx, 4);
        // assert_eq!(node.len(), 3);
        // assert_eq!(node[0][0], Value::Identifier(Rc::new("+".into())));
        // assert_eq!(node[0].len(), 1);
        // assert_eq!(node[1][0], Value::Int(2));
        // assert_eq!(node[1].len(), 1);
        // assert_eq!(node[2][0], Value::Int(2));
        // assert_eq!(node[2].len(), 1);
    }
}

//     #[test]
//     fn test_parsing_nested() {
//         let tokens = vec![
//             TokenData::new("(", 0, 0),
//             TokenData::new("+", 0, 0),
//             TokenData::new("2", 0, 0),
//             TokenData::new("(", 0, 0),
//             TokenData::new("+", 0, 0),
//             TokenData::new("2", 0, 0),
//             TokenData::new("2", 0, 0),
//             TokenData::new(")", 0, 0),
//             TokenData::new(")", 0, 0),
//         ];
//         let (node, next_idx) = preparse(&tokens, 0);

//         panic!("{:?}", node);
//         assert_eq!(node.len(), 3);
//     }
// }
