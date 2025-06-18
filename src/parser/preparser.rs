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

    /// Append a ValueList on to the end of self.
    pub fn append(&self, item: ValueList) -> ValueList {
        match self {
            ValueList::Empty => item,
            ValueList::Value(value) => ValueList::List(vec![ValueList::Value(value.clone()), item]),
            ValueList::List(_) => {
                let mut m = self.clone().unravel();
                m.push(item);
                ValueList::List(m)
            }
        }
    }

    // Useful for tests. Maybe useful elsewhere as well.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        match self {
            ValueList::Empty => 0,
            ValueList::Value(_) => 1,
            ValueList::List(list) => list.len(),
        }
    }

    /// Return the inner value as a Vec of ValueLists
    pub fn unravel(&self) -> Vec<ValueList> {
        match self {
            ValueList::Empty => vec![ValueList::Empty],
            ValueList::Value(_) => vec![self.clone()],
            ValueList::List(list) => list.clone(),
        }
    }
}

/// This produces a perfectly flat representation of the input source code.
/// It's kind of like just replacing the paretheses in source with the brackets
/// of arrays.
pub fn preparse(tokens: &[TokenData], mut idx: usize) -> (ValueList, usize) {
    let mut values: ValueList = ValueList::Empty;
    let mut current_list: ValueList = ValueList::Empty;

    while idx < tokens.len() {
        match &tokens[idx].token {
            Token::Value(value) => {
                current_list = current_list.push(value.clone());
            }
            Token::OpenParenthesis => {
                idx += 1;
                let (inner_list, new_idx) = preparse(tokens, idx);
                current_list = current_list.append(inner_list);
                idx = new_idx;
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
            TokenData::new("+", 0, 0),
            TokenData::new("1", 0, 0),
            TokenData::new("2", 0, 0),
            TokenData::new(")", 0, 0),
        ];
        let (value_list, next_idx) = preparse(&tokens, 0);

        assert_eq!(next_idx, 5);
        assert_eq!(value_list.len(), 3);
        assert_eq!(
            value_list.unravel()[0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list.unravel()[1], ValueList::Value(Value::Int(1)));
        assert_eq!(value_list.unravel()[2], ValueList::Value(Value::Int(2)));
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
        assert_eq!(value_list.len(), 3);
        assert_eq!(
            value_list.unravel()[0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(value_list.unravel()[1], ValueList::Value(Value::Int(1)));
        assert_eq!(value_list.unravel()[2].len(), 3);
        assert_eq!(
            value_list.unravel()[2].unravel()[0],
            ValueList::Value(Value::Identifier(Rc::new("+".into())))
        );
        assert_eq!(
            value_list.unravel()[2].unravel()[1],
            ValueList::Value(Value::Int(2))
        );
        assert_eq!(
            value_list.unravel()[2].unravel()[2],
            ValueList::Value(Value::Int(3))
        );
    }

    // #[test]
    // fn test_parsing_multiple() {
    //     let tokens = vec![
    //         TokenData::new("(", 0, 0),
    //         TokenData::new("(", 0, 0),
    //         TokenData::new("+", 0, 0),
    //         TokenData::new("1", 0, 0),
    //         TokenData::new("2", 0, 0),
    //         TokenData::new(")", 0, 0),
    //         TokenData::new("(", 0, 0),
    //         TokenData::new("+", 0, 0),
    //         TokenData::new("3", 0, 0),
    //         TokenData::new("4", 0, 0),
    //         TokenData::new(")", 0, 0),
    //         TokenData::new(")", 0, 0),
    //     ];
    //     let (value_list, next_idx) = preparse(&tokens, 0);

    //     assert_eq!(next_idx, 12);
    //     panic!("{:#?}", value_list);
    //     assert_eq!(value_list.len(), 2);
    //     assert_eq!(
    //         value_list.unravel()[0],
    //         ValueList::Value(Value::Identifier(Rc::new("+".into())))
    //     );
    //     assert_eq!(value_list.unravel()[1], ValueList::Value(Value::Int(1)));
    //     assert_eq!(value_list.unravel()[2], ValueList::Value(Value::Int(2)));
    // }
}
