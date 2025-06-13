use crate::tokenizer::TokenData;
use std::collections::HashMap;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};
use std::{default, fmt};

use crate::tokenizer::{Token, Value};

#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    pub operator: Value,
    pub operands: Vec<Operand>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            operator: Value::Unit,
            operands: Vec::new(),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
enum Operand {
    Value(Value),
    Node(Node),
}

pub(crate) fn expressionize(tokens: &[TokenData], idx: usize) -> (Node, usize) {
    let mut local_idx = idx;

    let mut node = Node::default();
    while local_idx < tokens.len() {
        println!("{:?}", &tokens[idx].token);

        match &tokens[local_idx].token {
            Token::Value(value) => match value {
                Value::Int(int) => {
                    if node.operator == Value::Unit {
                        todo!("not valid  with operator")
                    } else {
                        node.operands.push(Operand::Value(Value::Int(*int)));
                    }
                }
                Value::Identifier(identifier) => {
                    if node.operator == Value::Unit {
                        node.operator = Value::Identifier(identifier.clone());
                    } else {
                        node.operands
                            .push(Operand::Value(Value::Identifier(identifier.clone())));
                    }
                }
                Value::Unit => {
                    if node.operator == Value::Unit {
                        todo!("not valid  with operator")
                    } else {
                        node.operands.push(Operand::Value(Value::Unit));
                    }
                }
            },
            Token::OpenParenthesis => {
                local_idx += 1;
                let inner_nodes = expressionize(tokens, local_idx);
                node.operands.push(Operand::Node(inner_nodes.0));
                local_idx = inner_nodes.1;
                continue;
            }
            Token::CloseParenthesis => return (node, local_idx + 1),
            Token::TokenizationError(_) => todo!(),
        }
        local_idx += 1;
    }
    (node, local_idx)
}
