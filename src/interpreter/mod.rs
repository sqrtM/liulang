use std::rc::Rc;

use crate::{
    analyzer::Node,
    parser::{Operator, Token, Value},
};

pub fn execute(t: Rc<Node>) -> Value {
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
