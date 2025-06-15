use crate::parser::preparser::ValueList;
use std::fmt::Debug;

use crate::tokenizer::Value;
//pub mod flattener;
pub mod preparser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    Operation(OperationNode),
    List(Vec<Operand>),
    Empty,
}

impl Node {
    fn set_operator(&mut self, operator: Value) -> Self {
        Self::Operation(OperationNode {
            operator,
            operands: self.get_operands(),
        })
    }

    fn get_operands(&self) -> Vec<Operand> {
        match self {
            Node::Operation(operation_node) => operation_node.operands.clone(),
            Node::List(operands) => operands.to_vec(),
            Node::Empty => Vec::new(),
        }
    }

    fn push_operand(&self, operand: Operand) -> Self {
        let mut joined_operands = self.get_operands().clone();
        joined_operands.push(operand);
        match self {
            Node::Operation(operation_node) => Self::Operation(OperationNode {
                operator: operation_node.operator.clone(),
                operands: joined_operands,
            }),
            _ => Self::List(joined_operands),
        }
    }

    fn get_operator(&self) -> Value {
        match self {
            Node::Operation(operation_node) => operation_node.operator.clone(),
            Node::List(_) => Value::Unit,
            Node::Empty => Value::Unit,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OperationNode {
    pub operator: Value,
    pub operands: Vec<Operand>,
}

impl Default for OperationNode {
    fn default() -> Self {
        Self {
            operator: Value::Unit,
            operands: Vec::new(),
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operand {
    Value(Value),
    Node(Node),
}

pub(crate) fn parse(listed_values: ValueList) -> Node {
    let mut node = Node::default();
    for inner_value in &listed_values.unravel() {
        node = match inner_value {
            ValueList::Value(value) => match value {
                Value::Int(int) => node.push_operand(Operand::Value(Value::Int(*int))),
                Value::Identifier(identifier) => {
                    // This is kind of hokey and hard to get right. Checking for a valid operator
                    // during this phase makes it difficult to know what new operators will be
                    // added. Try and make this a bit more elegant.
                    if node.get_operator() == Value::Unit && is_valid_operator(&identifier) {
                        node.set_operator(Value::Identifier(identifier.clone()))
                    } else {
                        node.push_operand(Operand::Value(Value::Identifier(identifier.clone())))
                    }
                }
                Value::Unit => node.push_operand(Operand::Value(Value::Unit)),
            },
            ValueList::List(list) => {
                let inner_nodes = parse(ValueList::List(list.clone()));
                node.push_operand(Operand::Node(inner_nodes))
            }
            _ => todo!(),
        };
    }
    node
}

fn is_valid_operator(op: &String) -> bool {
    matches!(op.as_str(), "+")
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_parsing() {
        let list = ValueList::List(vec![
            ValueList::Value(Value::Identifier(Rc::new("+".into()))),
            ValueList::Value(Value::Int(1)),
            ValueList::Value(Value::Int(2)),
        ]);
        let value_list = parse(list);

        panic!("{:#?}", value_list);
    }

    #[test]
    fn test_parsing_def() {
        let list = ValueList::List(vec![
            ValueList::Value(Value::Identifier(Rc::new("def".into()))),
            ValueList::Value(Value::Identifier(Rc::new("add".into()))),
            ValueList::List(vec![
                ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                ValueList::Value(Value::Identifier(Rc::new("b".into()))),
            ]),
            ValueList::List(vec![
                ValueList::Value(Value::Identifier(Rc::new("+".into()))),
                ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                ValueList::Value(Value::Identifier(Rc::new("b".into()))),
            ]),
        ]);
        let value_list = parse(list);

        panic!("{:#?}", value_list);
    }
}
