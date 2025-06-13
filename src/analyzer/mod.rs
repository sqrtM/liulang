use crate::parser::TokenData;
use std::collections::HashMap;
use std::fmt;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use crate::parser::{Token, Value};

#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    pub operator: Value,
    pub operands: Vec<Value>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            operator: Value::Unit,
            operands: Vec::new(),
        }
    }
}
