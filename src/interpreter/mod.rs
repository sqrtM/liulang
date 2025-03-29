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
            Value::Unit => value,
            Value::Int(_) => value,
            Value::Identifier(ref name) => {
                match &t.parent {
                    Some(par) => match &par.token {
                        Token::Keyword(keyword) => match keyword {
                            Keyword::Def => {
                                // We are currently defining this value,
                                // so return it as it.
                                if t.parent
                                    .as_ref()
                                    .unwrap()
                                    .children
                                    .borrow()
                                    .first()
                                    .unwrap()
                                    .token
                                    == t.token
                                {
                                    value
                                } else {
                                    match resolve_variable(name, &t.context) {
                                        Some(val) => val.clone(),
                                        None => {
                                            panic!("{:?}", t);
                                        }
                                    }
                                }
                            }
                        },
                        _ => match resolve_variable(name, &t.context) {
                            Some(val) => val.clone(),
                            None => {
                                panic!("{:?}", t);
                            }
                        },
                    },
                    None => todo!("couldnt find the parent ? that doesn't make logical sense"),
                }
            }
            Value::Expression(_, _) => value,
        },
        Token::Keyword(keyword) => match keyword {
            Keyword::Def => {
                println!("beep");
                let symbol = flatten(children.first().unwrap());
                let identifier = match symbol {
                    Value::Identifier(identifier) => identifier,
                    _ => todo!("unexpected symbol"),
                };

                let args: Option<Value> = match children.get(1) {
                    Some(arg) => match Some(flatten(arg)) {
                        Some(x) => match x {
                            Value::Unit => Some(Value::Unit),
                            _ => todo!("only units for now"),
                        },
                        None => todo!(" no args ???"),
                    },
                    None => None,
                };

                let inner_args = match args {
                    Some(x) => x,
                    None => todo!(),
                };

                println!("flattening {:?}", children.get(2).unwrap());

                let value = match children.get(2) {
                    Some(child) => flatten(child),
                    None => todo!(),
                };

                t.context
                    .parent
                    .as_ref()
                    .unwrap()
                    .context
                    .borrow_mut()
                    .insert(identifier.to_string(), value);

                println!("defining {:?} in {:?}", identifier, t.context.parent);

                Value::Expression(vec![Rc::new(inner_args)], t.context.clone())
            }
        },
        Token::Unit => Value::Unit,
        _ => todo!(),
    }
}

fn resolve_variable(name: &String, node: &Rc<CtxNode>) -> Option<Value> {
    println!("looking for {:?} in {:?}", name, node);

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
