use std::{
    io::{self, Write},
    rc::Rc,
};

use crate::analyzer::Node;

pub fn evaluate() -> Vec<Rc<Node>> {
    init_terminal();

    //expressionize_input(Vec::new())
    todo!()
}

fn init_terminal() {
    println!(
        "\x1b[1;34mliulang\x1b[1;0m \x1b[4;34mREPL\x1b[1;0m\x1b[1;33m 流浪\x1b[1;0m \x1b[34mv.0.0.1 🦀 v1.85.0"
    );
    io::stdout().flush().unwrap();
}
/*
fn handle_input() -> String {
    print!("\x1b[1;0m> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn expressionize_input(mut tokens: Vec<TokenData>) -> Rc<CtxNode> {
    let input = handle_input();
    tokens.append(&mut parser::tokenize(input.trim(), 0));
    let result = analyzer::expressionize(&tokens);

    if result.as_ref().children.borrow().len() > 0 {
        result
    } else {
        expressionize_input(tokens)
    }
}
 */
