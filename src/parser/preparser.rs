use crate::tokenizer::TokenData;

use crate::tokenizer::{Token, Value};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueList {
    Empty,
    Value(Value),
    List(Vec<ValueList>),
}

impl ValueList {
    /// Push a Value into the current ValueList.
    pub fn push(&self, item: Value) -> ValueList {
        match self {
            ValueList::Empty => ValueList::Value(item),
            ValueList::Value(value) => ValueList::List(vec![
                ValueList::Value(value.clone()),
                ValueList::Value(item),
            ]),
            ValueList::List(list) => {
                let mut val = list.clone();
                val.push(ValueList::Value(item));
                ValueList::List(val)
            }
        }
    }

    // Push a ValueList into the current ValueList
    pub fn extend(&self, item: ValueList) -> ValueList {
        match self {
            ValueList::Empty => ValueList::List(vec![item]),
            ValueList::Value(value) => ValueList::List(vec![ValueList::Value(value.clone()), item]),
            ValueList::List(list) => {
                let mut m = list.clone();
                m.push(item);
                ValueList::List(m)
            }
        }
    }

    /// Return the inner value as a Vec of ValueLists
    pub fn unravel(&self) -> Vec<&ValueList> {
        match self {
            ValueList::Empty => vec![&ValueList::Empty],
            ValueList::Value(_) => vec![self],
            ValueList::List(list) => list.iter().collect(),
        }
    }
}

/// This produces a perfectly flat representation of the input source code.
/// It's kind of like just replacing the paretheses in source with the brackets
/// of arrays.
pub fn preparse(tokens: &[TokenData], mut idx: usize) -> (ValueList, usize) {
    let mut current_list: ValueList = ValueList::Empty;

    while idx < tokens.len() {
        match &tokens[idx].token {
            Token::Value(value) => {
                current_list = current_list.push(value.clone());
            }
            Token::OpenParenthesis => {
                let (inner_list, new_idx) = preparse(tokens, idx + 1);

                current_list = current_list.extend(inner_list);
                idx = new_idx;
                continue;
            }
            Token::CloseParenthesis => {
                idx += 1;
                break;
            }
            Token::TokenizationError(_) => todo!(),
        };
        idx += 1;
    }

    (current_list, idx)
}

#[cfg(test)]
mod tests {
    use std::{ops::Index, rc::Rc};

    use super::*;

    impl Index<usize> for ValueList {
        type Output = Self;

        fn index(&self, index: usize) -> &Self::Output {
            self.unravel()[index]
        }
    }

    impl ValueList {
        fn len(&self) -> usize {
            match self {
                ValueList::Empty => 0,
                ValueList::Value(_) => 1,
                ValueList::List(list) => list.len(),
            }
        }
    }

    #[test]
    fn test_parsing() {
        let tokens = vec![
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("1", 0, 0),
            TokenData::new("2", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (value_list, next_idx) = preparse(&tokens, 0);

        assert_eq!(next_idx, 5);
        assert_eq!(value_list.len(), 1);
        assert_eq!(value_list[0].len(), 3);
        assert_eq!(
            value_list[0][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list[0][1], ValueList::Value(Value::Int(1)));
        assert_eq!(value_list[0][2], ValueList::Value(Value::Int(2)));
    }

    #[test]
    fn test_parsing_nested() {
        let tokens = vec![
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("1", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("2", 0, 0),
            TokenData::new("3", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (value_list, next_idx) = preparse(&tokens, 0);

        assert_eq!(next_idx, 9);
        assert_eq!(value_list.len(), 1);
        assert_eq!(
            value_list[0][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list[0][1], ValueList::Value(Value::Int(1)));
        assert_eq!(value_list[0][2].len(), 3);
        assert_eq!(
            value_list[0][2][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list[0][2][1], ValueList::Value(Value::Int(2)));
        assert_eq!(value_list[0][2][2], ValueList::Value(Value::Int(3)));
    }

    #[test]
    fn test_parsing_multiple() {
        let tokens = vec![
            TokenData::new("(", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("1", 0, 0),
            TokenData::new("2", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("3", 0, 0),
            TokenData::new("4", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (value_list, next_idx) = preparse(&tokens, 0);

        assert_eq!(next_idx, 12);
        assert_eq!(value_list.len(), 1);

        assert_eq!(value_list[0].len(), 2);
        assert_eq!(value_list[0][0].len(), 3);
        assert_eq!(
            value_list[0][0][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list[0][0][1], ValueList::Value(Value::Int(1)));
        assert_eq!(value_list[0][0][2], ValueList::Value(Value::Int(2)));

        assert_eq!(value_list[0][1].len(), 3);
        assert_eq!(
            value_list[0][1][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list[0][1][1], ValueList::Value(Value::Int(3)));
        assert_eq!(value_list[0][1][2], ValueList::Value(Value::Int(4)));
    }

    #[test]
    fn test_parsing_def_function() {
        let tokens = vec![
            TokenData::new("(", 0, 0),
            TokenData::new("def", 0, 0),
            TokenData::new("add", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("a", 0, 0),
            TokenData::new("b", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new("(", 0, 0),
            TokenData::new("+", 0, 0),
            TokenData::new("a", 0, 0),
            TokenData::new("b", 0, 0),
            TokenData::new(")", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (value_list, next_idx) = preparse(&tokens, 0);

        assert_eq!(next_idx, 13);
        assert_eq!(value_list.len(), 1);

        assert_eq!(value_list[0].len(), 4);
        assert_eq!(value_list[0][0].len(), 1);
        assert_eq!(
            value_list[0][0],
            ValueList::Value(Value::Identifier(Rc::new("def".into())))
        );

        assert_eq!(value_list[0][1].len(), 1);
        assert_eq!(
            value_list[0][1],
            ValueList::Value(Value::Identifier(Rc::new("add".into())))
        );

        assert_eq!(value_list[0][2].len(), 2);
        assert_eq!(
            value_list[0][2][0],
            ValueList::Value(Value::Identifier(Rc::new("a".into())))
        );
        assert_eq!(
            value_list[0][2][1],
            ValueList::Value(Value::Identifier(Rc::new("b".into())))
        );

        assert_eq!(value_list[0][3].len(), 3);
        assert_eq!(
            value_list[0][3][0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(
            value_list[0][3][1],
            ValueList::Value(Value::Identifier(Rc::new("a".into())))
        );
        assert_eq!(
            value_list[0][3][2],
            ValueList::Value(Value::Identifier(Rc::new("b".into())))
        );
    }
}
