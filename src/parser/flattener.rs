use std::{collections::HashMap, rc::Rc};

use crate::{
    parser::{self, Operand, OperationNode},
    tokenizer::Value,
};

pub fn flatten(node: &OperationNode) -> OperationNode {
    let operands = node
        .operands
        .iter()
        .map(|operand| match operand {
            parser::Operand::Value(value) => parser::Operand::Value(value.clone()),
            parser::Operand::Node(node) => parser::Operand::Node(flatten(node)),
        })
        .collect();
    OperationNode {
        operator: node.operator.clone(),
        operands,
    }
}

pub fn define_identifiers(node: &OperationNode) -> OperationNode {
    // match node.operator {
    //     Value::Int(_) => todo!("this probably should not even be possible..."),
    //     Value::Identifier(identifier) => if identifier == "def".to_string().into() {},
    //     Value::Unit => todo!(),
    // };

    let operands = node
        .operands
        .iter()
        .map(|operand| match operand {
            parser::Operand::Value(value) => parser::Operand::Value(value.clone()),
            parser::Operand::Node(node) => parser::Operand::Node(flatten(node)),
        })
        .collect();
    OperationNode {
        operator: node.operator.clone(),
        operands,
    }
}

struct SymbolDefinition {
    args: Vec<String>,
    def: OperationNode,
}

fn hjhdsfkh() {
    let mut symbol_table: HashMap<String, SymbolDefinition> = HashMap::new();
    let args = vec!["a".to_string(), "b".to_string()];
    let def = OperationNode {
        operator: Value::Identifier(Rc::new("+".into())),
        operands: vec![
            Operand::Value(Value::Identifier(Rc::new("a".into()))),
            Operand::Value(Value::Identifier(Rc::new("b".into()))),
        ],
    };
    let definition = SymbolDefinition { args, def };
    symbol_table.insert("add".into(), definition);

    todo!()
}
