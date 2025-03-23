use crate::TokenData;
use std::collections::HashMap;
use std::fmt;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use crate::parser::{Keyword, Token, Value};

#[derive(PartialEq, Eq)]
pub struct Node {
    pub token: Token,
    pub parent: Option<Rc<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub context: Rc<CtxNode>,
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

#[derive(PartialEq, Eq)]
pub struct CtxNode {
    pub id: usize,
    pub context: RefCell<HashMap<String, Value>>,
    pub parent: Option<Rc<CtxNode>>,
    pub children: RefCell<Vec<Rc<CtxNode>>>,
    pub expression: RefCell<Option<Rc<Node>>>,
}

impl Debug for CtxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let p = self.parent.as_ref().map(|p| p.id);

        let c = self.children.borrow();
        let childs = c.iter().map(|child| child.id).collect::<Vec<usize>>();

        write!(
            f,
            "Node {{ ID: {:?}, parent: {:?}, children: {:?} }}",
            self.id, p, childs
        )
    }
}

pub fn expressionize(tokens: &Vec<TokenData>) -> Rc<CtxNode> {
    let mut current_node: Option<Rc<Node>> = None;
    let mut depth = 0;
    let mut nodes: Vec<Rc<Node>> = Vec::new();
    let mut index = 0;

    let mut ctxnode = Rc::new(CtxNode {
        id: index,
        context: RefCell::new(HashMap::new()),
        parent: None,
        children: RefCell::new(vec![]),
        expression: RefCell::new(None),
    });

    for token_data in tokens {
        match token_data.token.clone() {
            Token::Operator(operator) => {
                let new_node = Rc::new(Node {
                    token: Token::Operator(operator),
                    parent: current_node.clone(),
                    children: RefCell::new(vec![]),
                    context: ctxnode.clone(),
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
                    context: ctxnode.clone(),
                });

                if let Some(ref c_n) = current_node {
                    c_n.children.borrow_mut().push(new_node.clone());
                }
            }
            Token::OpenParenthesis => {
                depth += 1;
                index += 1;
                let new_ctxnode = CtxNode {
                    id: index,
                    context: RefCell::new(HashMap::new()),
                    parent: Some(ctxnode),
                    children: RefCell::new(vec![]),
                    expression: RefCell::new(None),
                };

                ctxnode = Rc::new(new_ctxnode);
            }
            Token::CloseParenthesis => {
                depth -= 1;
                if depth > 0 {
                    if let Some(c_n) = current_node {
                        if c_n.get_parent().is_none() {
                            nodes.push(c_n.clone());
                        }
                        ctxnode.expression.swap(&RefCell::new(Some(c_n.clone())));
                        current_node = c_n.get_parent()
                    }

                    if let Some(ref cur_ctx_node) = ctxnode.parent {
                        if cur_ctx_node.parent.is_none() {
                            todo!()
                        } else {
                            ctxnode
                                .parent
                                .as_ref()
                                .unwrap()
                                .children
                                .borrow_mut()
                                .push(ctxnode.clone());
                            ctxnode = ctxnode.parent.clone().unwrap()
                        }
                    }
                }
                if depth < 0 {
                    todo!("error out too many closing parentheses")
                }
            }
            Token::Keyword(keyword) => match keyword {
                Keyword::Def => {
                    let new_node = Rc::new(Node {
                        token: Token::Keyword(Keyword::Def),
                        parent: current_node.clone(),
                        children: RefCell::new(vec![]),
                        context: ctxnode.clone(),
                    });

                    if let Some(ref c_n) = current_node {
                        c_n.children.borrow_mut().push(new_node.clone());
                    }
                    current_node = Some(new_node);
                }
            },
            _ => todo!(),
        }
    }

    if depth != 0 {
        todo!("bad parentheses")
    } else {
        ctxnode
    }
}
