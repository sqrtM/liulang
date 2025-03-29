use std::rc::Rc;

use crate::{
    analyzer::{CtxNode, Node},
    parser::{Keyword, Operator, Token, Value},
};

pub fn flatten(t: &Rc<Node>) -> Value {
    let children = t.children.borrow();

    match t.token.clone() {
        Token::Operator(operator) => apply_operator(&operator, &children),
        Token::Value(value) => match value {
            Value::Identifier(ref name) => {
                if let Some(par) = &t.parent {
                    if let Token::Keyword(Keyword::Def) = par.token {
                        // We are currently defining this value, so return it as it.
                        if par.children.borrow().first().map(|c| &c.token) == Some(&t.token) {
                            return value;
                        }
                    }
                }

                resolve_variable(name, &t.context).unwrap_or_else(|| panic!("{:?}", t))
            }
            _ => value,
        },

        Token::Keyword(Keyword::Def) => {
            let symbol = flatten(children.first().unwrap());
            let identifier = match symbol {
                Value::Identifier(identifier) => identifier,
                _ => todo!("unexpected symbol"),
            };

            let args = children.get(1).map(|arg| match flatten(arg) {
                Value::Unit => Value::Unit,
                _ => todo!("only units for now"),
            });

            let value = children.get(2).map(flatten).unwrap_or_else(|| todo!());

            if let Some(parent_context) = &t.context.parent {
                parent_context
                    .context
                    .borrow_mut()
                    .insert(identifier.to_string(), value);
            } else {
                todo!("No parent context found");
            }

            Value::Expression(vec![Rc::new(args.unwrap())], t.context.clone())
        }
        Token::Unit => Value::Unit,
        _ => todo!(),
    }
}

fn apply_operator(operator: &Operator, children: &[Rc<Node>]) -> Value {
    let mut iter = children.iter();
    let first = flatten(iter.next().unwrap());

    match operator {
        Operator::Plus => iter.fold(first, |acc, child| acc + flatten(child)),
        Operator::Minus => iter.fold(first, |acc, child| acc - flatten(child)),
        //_ => todo!(),
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
