use std::rc::Rc;

use crate::{
    parser::{Node, Operand},
    tokenizer::Value,
};

pub(crate) fn find_operator(node: &Node) -> Option<String> {
    if node.get_operator() == Value::Identifier(Rc::new("def".into()))
        && node.get_operands().len() == 3
    {
        let operands = node.get_operands();

        let identifier_node = operands.get(0);
        let args_node = operands.get(1);
        let definition_node = operands.get(2);

        let identifier = identifier_node.and_then(|id| match id {
            Operand::Value(value) => match value {
                Value::Int(_) => todo!("not valid 2"),
                Value::Identifier(s) => Some(s),
                Value::Unit => None,
            },
            Operand::Node(n) => todo!("not valid 1 == {:?}", n),
        });

        let _args_node_is_valid = args_node.and_then(|args| match args {
            Operand::Value(_) => todo!("not valid yet(?)"),
            Operand::Node(node) => match node {
                Node::Operation(_) => todo!("not valid"),
                Node::List(operands) => Some(operands),
                Node::Empty => None,
            },
        });

        let _definition_node_is_valid = definition_node.and_then(|args| match args {
            Operand::Value(_) => todo!("not valid yet(?)"),
            Operand::Node(node) => match node {
                Node::Operation(operation_node) => Some(operation_node),
                Node::List(_) => None,
                Node::Empty => None,
            },
        });

        if _args_node_is_valid
            .map(|args| !args.is_empty())
            .unwrap_or(false)
        {
            return identifier.and_then(|id| Some(id.as_str().to_string()));
        }
        None
    } else {
        None
    }
}
