#![feature(extern_types)]

use std::io::{self, BufRead, Write};

mod lexer;
use lexer::TokenReader;

mod parser;
use parser::{AstNode, FunctionAstNode, Parser};

mod codegen;
use codegen::CodegenContext;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();

    let mut buf = String::new();
    loop {
        buf.clear();
        print!("ready> ");
        stdout.flush().unwrap();

        stdin.read_line(&mut buf).unwrap();
        let token_reader = TokenReader::new(buf.chars());
        let mut parser = Parser::new(token_reader);

        let expr = parser.parse_top_level();
        let mut codegen = CodegenContext::new();
        let number = match expr.unwrap() {
            AstNode::Function(FunctionAstNode { body, .. }) => body,
            _ => unreachable!(),
        };
        let value = codegen.codegen(*number);
        codegen::print_value(value);
        print!("\n");
        stdout.flush().unwrap();
    }
}
