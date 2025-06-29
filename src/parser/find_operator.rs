use std::rc::Rc;

use crate::{
    parser::{Node, Operand},
    tokenizer::Value,
};

pub(crate) fn find_operator(node: &Node) -> Option<String> {
    let operands = node.get_operands();

    let identifier_node = operands.first();
    let args_node = operands.get(1);
    let definition_node = operands.get(2);

    if node.get_operator() == Value::Identifier(Rc::new("def".into()))
        && node.get_operands().len() == 3
        && let Some(Operand::Value(Value::Identifier(identifier))) = identifier_node
        && let Some(Operand::Node(Node::List(_args))) = args_node
        && let Some(Operand::Node(Node::Operation(_function_definition))) = definition_node
    {
        Some(identifier.to_string())
    } else {
        None
    }
}
