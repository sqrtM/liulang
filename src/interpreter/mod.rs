use std::rc::Rc;

use crate::{
    analyzer::Node,
    parser::{Keyword, Operator, Token, Value},
};

pub fn flatten(t: Rc<Node>) -> Value {
    let children = t.children.borrow();
    match t.token {
        Token::Operator(operator) => match operator {
            Operator::Plus => children
                .iter()
                .fold(Value::Int(0), |acc, child| acc + flatten(child.clone())),
            Operator::Minus => children
                .iter()
                .skip(1)
                .fold(flatten(children[0].clone()), |acc, child| {
                    acc - flatten(child.clone())
                }),
        },
        Token::Value(value) => value,
        Token::Keyword(keyword) => match keyword {
            Keyword::Defun => Value::Int(0),
        },
        _ => todo!(),
    }
}
