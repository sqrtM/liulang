use crate::parser::{find_operator::find_operator, preparser::ValueList};
use std::fmt::Debug;

use crate::tokenizer::Value;
//pub mod flattener;
pub mod find_operator;
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

/// This takes in the preparsed flat input and transforms it into
/// a parce tree, beginning with an entry point node (currently just
/// the first expression in the source code) and producing parent-child
/// relationships across all other expressions.
pub(crate) fn parse(listed_values: ValueList, mut symbols: Vec<String>) -> (Node, Option<String>) {
    let mut node = Node::default();
    for inner_value in &listed_values.unravel() {
        node = match inner_value {
            ValueList::Value(value) => match value {
                Value::Int(int) => node.push_operand(Operand::Value(Value::Int(*int))),
                Value::Identifier(identifier) => {
                    // This is kind of hokey and hard to get right. Checking for a valid operator
                    // during this phase makes it difficult to know what new operators will be
                    // added. Try and make this a bit more elegant.
                    if node.get_operator() == Value::Unit && is_valid_operator(identifier, &symbols)
                    {
                        node.set_operator(Value::Identifier(identifier.clone()))
                    } else {
                        node.push_operand(Operand::Value(Value::Identifier(identifier.clone())))
                    }
                }
                Value::Unit => node.push_operand(Operand::Value(Value::Unit)),
            },
            ValueList::List(list) => {
                let (inner_nodes, maybe_symbol) =
                    parse(ValueList::List(list.clone()), symbols.clone());
                if let Some(symbol) = maybe_symbol {
                    symbols.push(symbol);
                }
                node.push_operand(Operand::Node(inner_nodes))
            }
            _ => todo!(),
        };
    }
    (node.clone(), find_operator(&node))
}

fn is_valid_operator(op: &str, symbols: &[String]) -> bool {
    matches!(op, "+" | "-" | "def") || symbols.iter().any(|s| s == op)
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_parsing() {
        let list = ValueList::List(vec![
            ValueList::Value(Value::Identifier(Rc::new("+".into()))),
            ValueList::Value(Value::Int(1)),
            ValueList::Value(Value::Int(2)),
        ]);
        let (node, _) = parse(list, Vec::new());

        assert_eq!(node.get_operator(), Value::Identifier(Rc::new("+".into())));
        assert_eq!(node.get_operands().len(), 2);
        assert_eq!(node.get_operands()[0], Operand::Value(Value::Int(1)));
        assert_eq!(node.get_operands()[1], Operand::Value(Value::Int(2)));
    }

    #[test]
    fn test_parsing_def() {
        // And here, we can test a global function table, and make sure that add is
        // added (ha ha) to the global table, and then we can use it in place of the
        // + in the test.
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
        let (node, _) = parse(list, Vec::new());

        assert_eq!(
            node.get_operator(),
            Value::Identifier(Rc::new("def".into()))
        );
        assert_eq!(node.get_operands().len(), 3);
        assert_eq!(
            node.get_operands()[0],
            Operand::Value(Value::Identifier(Rc::new("add".into())))
        );
        assert_eq!(
            node.get_operands()[1],
            Operand::Node(Node::List(vec![
                Operand::Value(Value::Identifier(Rc::new("a".into()))),
                Operand::Value(Value::Identifier(Rc::new("b".into())))
            ]))
        );
        assert_eq!(
            node.get_operands()[2],
            Operand::Node(Node::Operation(OperationNode {
                operator: Value::Identifier(Rc::new("+".into())),
                operands: vec![
                    Operand::Value(Value::Identifier(Rc::new("a".into()))),
                    Operand::Value(Value::Identifier(Rc::new("b".into())))
                ]
            }))
        );
    }

    #[test]
    fn test_parsing_def_scope() {
        // Defs should fall out of scope.
        let list = ValueList::List(vec![
            ValueList::List(vec![
                ValueList::List(vec![
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
                ]),
                ValueList::List(vec![
                    ValueList::Value(Value::Identifier(Rc::new("add".into()))), // Should be an operator
                    ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                    ValueList::Value(Value::Identifier(Rc::new("b".into()))),
                ]),
            ]),
            ValueList::List(vec![
                ValueList::List(vec![
                    ValueList::Value(Value::Identifier(Rc::new("def".into()))),
                    ValueList::Value(Value::Identifier(Rc::new("sub".into()))),
                    ValueList::List(vec![
                        ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                        ValueList::Value(Value::Identifier(Rc::new("b".into()))),
                    ]),
                    ValueList::List(vec![
                        ValueList::Value(Value::Identifier(Rc::new("-".into()))),
                        ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                        ValueList::Value(Value::Identifier(Rc::new("b".into()))),
                    ]),
                ]),
                ValueList::List(vec![
                    ValueList::Value(Value::Identifier(Rc::new("add".into()))), // Should not be an operator
                    ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                    ValueList::Value(Value::Identifier(Rc::new("b".into()))),
                ]),
                ValueList::List(vec![
                    ValueList::Value(Value::Identifier(Rc::new("sub".into()))), // Should be an operator
                    ValueList::Value(Value::Identifier(Rc::new("a".into()))),
                    ValueList::Value(Value::Identifier(Rc::new("b".into()))),
                ]),
            ]),
        ]);

        let (node, _) = parse(list, Vec::new());

        assert_eq!(node.get_operands().len(), 2);

        // Test first operand set
        match &node.get_operands()[0] {
            Operand::Value(value) => panic!("Should not be a value. Got {value:?}"),
            Operand::Node(inner_node) => {
                assert_eq!(inner_node.get_operands().len(), 2);
                assert_eq!(
                    inner_node.get_operands()[1],
                    Operand::Node(Node::Operation(OperationNode {
                        operator: Value::Identifier(Rc::new("add".into())),
                        operands: vec![
                            Operand::Value(Value::Identifier(Rc::new("a".into()))),
                            Operand::Value(Value::Identifier(Rc::new("b".into())))
                        ]
                    }))
                );
            }
        };

        // Test second operand set
        match &node.get_operands()[1] {
            Operand::Value(value) => panic!("Should not be a value. Got {value:?}"),
            Operand::Node(inner_node) => {
                assert_eq!(inner_node.get_operands().len(), 3);
                assert_eq!(
                    inner_node.get_operands()[1],
                    Operand::Node(Node::List(vec![
                        // This becomes a list because operator is out of scope.
                        Operand::Value(Value::Identifier(Rc::new("add".into()))),
                        Operand::Value(Value::Identifier(Rc::new("a".into()))),
                        Operand::Value(Value::Identifier(Rc::new("b".into())))
                    ]))
                );
                assert_eq!(
                    inner_node.get_operands()[2],
                    Operand::Node(Node::Operation(OperationNode {
                        operator: Value::Identifier(Rc::new("sub".into())),
                        operands: vec![
                            Operand::Value(Value::Identifier(Rc::new("a".into()))),
                            Operand::Value(Value::Identifier(Rc::new("b".into())))
                        ]
                    }))
                );
            }
        };
    }
}
