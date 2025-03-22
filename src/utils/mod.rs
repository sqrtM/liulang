use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    rc::Rc,
};

use crate::{
    analyzer::{self, Node},
    interpreter,
    parser::{self, TokenData, Value},
};

enum InputType {
    Repl,
    File(PathBuf),
}

enum ContextType {
    Input(InputContext),
    Parser(ParserContext),
    Analyzer(AnalyzerContext),
    Interpreter(InterpreterContext),
    Output(OutputContext),
}

struct InputContext {
    input_type: InputType,
}

impl InputContext {
    fn into(self, lines: Vec<String>) -> ParserContext {
        ParserContext {
            lines,
            input_context: self,
        }
    }
}

struct ParserContext {
    lines: Vec<String>,
    input_context: InputContext,
}

impl ParserContext {
    fn into(self, token_data: Vec<TokenData>) -> AnalyzerContext {
        AnalyzerContext {
            token_data,
            parser_context: self,
        }
    }
}

struct InterpreterContext {
    expression_data: (Vec<Rc<Node>>, HashMap<String, Node>),
    analyzer_context: AnalyzerContext,
}

impl AnalyzerContext {
    fn into(self, expression_data: (Vec<Rc<Node>>, HashMap<String, Node>)) -> InterpreterContext {
        InterpreterContext {
            expression_data,
            analyzer_context: self,
        }
    }
}

struct AnalyzerContext {
    token_data: Vec<TokenData>,
    parser_context: ParserContext,
}

impl InterpreterContext {
    fn into(self, values: Vec<Value>) -> OutputContext {
        OutputContext {
            values,
            //interpreter_context: self,
        }
    }
}

struct OutputContext {
    values: Vec<Value>,
    //interpreter_context: InterpreterContext,
}

pub struct Pipeline {
    ctx: ContextType,
}

impl Pipeline {
    pub fn new(path: PathBuf) -> Self {
        Self {
            ctx: ContextType::Input(InputContext {
                input_type: InputType::File(path),
            }),
        }
    }

    pub fn run(self) -> Self {
        match self.ctx {
            ContextType::Input(input_context) => match input_context.input_type {
                InputType::Repl => todo!(),
                InputType::File(ref path_buf) => {
                    let file = File::open(path_buf).unwrap();
                    let lines = io::BufReader::new(file)
                        .lines()
                        .map_while(Result::ok)
                        .collect();
                    Self {
                        ctx: ContextType::Parser(input_context.into(lines)),
                    }
                }
            },
            ContextType::Parser(parser_context) => {
                let tokens = parser_context
                    .lines
                    .iter()
                    .enumerate()
                    .flat_map(|(i, line)| parser::tokenize(line, i))
                    .collect();
                Self {
                    ctx: ContextType::Analyzer(parser_context.into(tokens)),
                }
            }
            ContextType::Analyzer(analyzer_context) => {
                let aaa = analyzer::expressionize(&analyzer_context.token_data);
                Self {
                    ctx: ContextType::Interpreter(analyzer_context.into(aaa)),
                }
            }
            ContextType::Interpreter(interpreter_context) => {
                let values = interpreter_context
                    .expression_data
                    .0
                    .iter()
                    .map(interpreter::flatten)
                    .collect();
                Self {
                    ctx: ContextType::Output(interpreter_context.into(values)),
                }
            }
            ContextType::Output(output_context) => {
                output_context
                    .values
                    .iter()
                    .for_each(|v| println!("{:?}", v));
                Self {
                    ctx: ContextType::Output(output_context),
                }
            }
        }
    }
}

pub fn show_license_notice() {
    println!("\x1b[1;30;47mliulang 流浪 - Copyright (C) 2025                            \x1b[1;0m");
    println!(
        "\x1b[30;47mThis program comes with \x1b[1;0m\x1b[1;30;47mABSOLUTELY NO WARRANTY.              \x1b[1;0m"
    );
    println!("\x1b[30;47mThis is free software, and you are welcome to redistribute it\x1b[1;0m");
    println!("\x1b[30;47munder certain conditions; see LICENSE for details.           \x1b[1;0m");
}
