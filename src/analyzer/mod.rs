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

impl Default for Node {
    fn default() -> Self {
        Self {
            token: Token::default(),
            parent: None,
            children: RefCell::new(Vec::new()),
            context: Rc::new(CtxNode::default()),
        }
    }
}

impl Node {
    fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.clone()
    }

    fn set_token(mut self, token: Token) -> Self {
        self.token = token;
        self
    }

    fn set_parent(mut self, parent: Option<Rc<Node>>) -> Self {
        self.parent = parent;
        self
    }

    fn set_context(mut self, ctx: Rc<CtxNode>) -> Self {
        self.context = ctx;
        self
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

impl Default for CtxNode {
    fn default() -> Self {
        Self {
            id: 0,
            context: RefCell::new(HashMap::new()),
            parent: None,
            children: RefCell::new(Vec::new()),
            expression: RefCell::new(None),
        }
    }
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

impl CtxNode {
    fn set_parent(mut self, parent: Option<Rc<CtxNode>>) -> Self {
        self.parent = parent;
        self
    }

    fn new(index: usize) -> Self {
        Self {
            id: index,
            ..Default::default()
        }
    }
}

pub fn expressionize(tokens: &[TokenData]) -> Rc<CtxNode> {
    let mut current_node: Option<Rc<Node>> = None;
    let mut current_parent_node: Option<Rc<Node>> = None;
    let mut depth = 0;
    let mut index = 0;

    let mut ctxnode = Rc::new(CtxNode::default());

    let mut idx = 0;

    loop {
        if idx >= tokens.len() {
            break;
        }

        match tokens[idx].token.clone() {
            Token::Operator(operator) => {
                if let Some(ref c_n) = current_parent_node {
                    let n = Node::default()
                        .set_token(Token::Operator(operator))
                        .set_context(ctxnode.clone());
                    let new_node = Rc::new(n.set_parent(Some(c_n.clone())));

                    c_n.children.borrow_mut().push(new_node.clone());
                    current_node = Some(new_node.clone());
                    current_parent_node = Some(new_node);
                } else {
                    let n = Node::default()
                        .set_token(Token::Operator(operator))
                        .set_context(ctxnode.clone());
                    let new_node = Rc::new(n);
                    current_node = Some(new_node.clone());
                    current_parent_node = Some(new_node);
                }
            }
            Token::Value(value) => {
                if let Some(ref c_n) = current_parent_node {
                    let new_node = Rc::new(
                        Node::default()
                            .set_token(Token::Value(value))
                            .set_context(ctxnode.clone())
                            .set_parent(current_parent_node.clone()),
                    );

                    c_n.children.borrow_mut().push(new_node.clone());
                    current_node = Some(new_node);
                } else {
                    let new_node = Node::default()
                        .set_token(Token::Value(value))
                        .set_context(ctxnode.clone());

                    current_node = Some(Rc::new(new_node));
                }
            }
            Token::OpenParenthesis => {
                if tokens[idx + 1].token == Token::CloseParenthesis {
                    // This will be a unit type. We can skip the next token
                    // And add the parent directly.

                    let new_node = Rc::new(
                        Node::default()
                            .set_parent(current_parent_node.clone())
                            .set_context(ctxnode.clone()),
                    );
                    if let Some(c_n) = current_parent_node.clone() {
                        c_n.children.borrow_mut().push(new_node.clone());
                    }
                    idx += 1;
                    current_node = Some(new_node);
                } else {
                    depth += 1;
                    index += 1;

                    let new_ctxnode =
                        Rc::new(CtxNode::new(index).set_parent(Some(ctxnode.clone())));
                    current_node = None;
                    ctxnode = new_ctxnode;
                }
            }
            Token::CloseParenthesis => {
                depth -= 1;

                if let Some(c_n) = current_node {
                    if c_n.get_parent().is_some() {
                        ctxnode
                            .expression
                            .swap(&RefCell::new(Some(c_n.get_parent().unwrap().clone())));
                    }
                    current_node = c_n.get_parent()
                }

                if let Some(ref par_ctx_node) = ctxnode.parent {
                    par_ctx_node.children.borrow_mut().push(ctxnode.clone());
                    ctxnode = ctxnode.parent.clone().unwrap();
                }

                if let Some(ref c_p_n) = current_parent_node {
                    if let Some(ref grand_parent_node) = c_p_n.get_parent() {
                        current_parent_node = Some(grand_parent_node.clone())
                    }
                }

                if depth < 0 {
                    todo!("error out too many closing parentheses")
                }
            }
            Token::Keyword(keyword) => match keyword {
                Keyword::Def => {
                    let new_node = Rc::new(
                        Node::default()
                            //.set_parent(current_node.clone()) -- not implemented
                            .set_token(Token::Keyword(Keyword::Def))
                            .set_context(ctxnode.clone()),
                    );

                    if current_node.is_some() {
                        panic!("not implemented")
                    }
                    current_node = Some(new_node.clone());
                    current_parent_node = Some(new_node);
                }
            },
            _ => todo!(),
        }
        idx += 1;
    }

    if depth != 0 {
        todo!("bad parentheses ; depth = {:?}", depth)
    } else {
        ctxnode
    }
}
