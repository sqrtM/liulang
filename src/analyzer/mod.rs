use crate::TokenData;
use std::collections::HashMap;
use std::fmt;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use crate::parser::Token;

#[derive(PartialEq, Eq)]
pub struct Node {
    pub token: Token,
    parent: Option<Rc<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.clone()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

pub fn expressionize(tokens: &Vec<TokenData>) -> (Vec<Rc<Node>>, HashMap<String, Node>) {
    let mut current_node: Option<Rc<Node>> = None;
    let mut depth = 0;
    let mut nodes: Vec<Rc<Node>> = Vec::new();

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
                }
                current_node = Some(new_node);
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
                    if let Some(c_n) = current_node {
                        if c_n.get_parent().is_none() {
                            nodes.push(c_n.clone());
                        }
                        current_node = c_n.get_parent()
                    }
                }
                if depth < 0 {
                    todo!("error out too many closing parentheses")
                }
            }
            _ => todo!(),
        }
    }

    if depth != 0 {
        (vec![], HashMap::new())
    } else {
        (nodes, HashMap::new())
    }
}
