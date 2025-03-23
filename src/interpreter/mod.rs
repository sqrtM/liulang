use std::rc::Rc;

use crate::{
    analyzer::{CtxNode, Node},
    parser::{Keyword, Operator, Token, Value},
};

pub fn flatten(t: &Rc<Node>) -> Value {
    let children = t.children.borrow();

    match t.token.clone() {
        Token::Operator(operator) => match operator {
            Operator::Plus => children
                .iter()
                .fold(Value::Int(0), |acc, child| acc + flatten(&child.clone())),
            Operator::Minus => children
                .iter()
                .skip(1)
                .fold(flatten(&children[0].clone()), |acc, child| {
                    acc - flatten(&child.clone())
                }),
        },
        Token::Value(value) => match value {
            Value::Int(_) => value,
            Value::Identifier(ref name) => match &t.parent {
                Some(par) => match &par.token {
                    Token::Keyword(keyword) => match keyword {
                        // We are currently defining this value,
                        // so return it as it.
                        Keyword::Def => value,
                    },
                    _ => match resolve_variable(name, &par.context) {
                        Some(val) => val.clone(),
                        None => {
                            todo!()
                        }
                    },
                },
                None => todo!(),
            },
            Value::Variable(_) => value,
        },
        Token::Keyword(keyword) => match keyword {
            Keyword::Def => {
                let symbol = flatten(children.first().unwrap());
                let s2 = symbol.clone();
                let identifier = match symbol {
                    Value::Identifier(identifier) => identifier,
                    _ => todo!("unexpected symbol"),
                };
                let value = flatten(children.get(1).unwrap());

                t.context
                    .parent
                    .as_ref()
                    .unwrap()
                    .context
                    .borrow_mut()
                    .insert(identifier.to_string(), value);
                Value::Variable(Rc::new(s2))
            }
        },
        _ => todo!(),
    }
}

fn resolve_variable(name: &String, node: &Rc<CtxNode>) -> Option<Value> {
    if let Some(parent) = &node.parent {
        if let Some(res) = parent.context.borrow().get(name) {
            Some(res.clone())
        } else {
            resolve_variable(name, node.parent.as_ref().unwrap())
        }
    } else {
        None
    }
}
