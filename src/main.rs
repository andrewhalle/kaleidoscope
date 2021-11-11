#![feature(extern_types)]

use std::io::{self, BufRead, Write};

mod lexer;
use lexer::TokenReader;

mod parser;
use parser::{AstNode, Parser};

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
        let func = match expr.unwrap() {
            AstNode::Function(function) => function,
            _ => unreachable!(),
        };
        let func = codegen.codegen_function(func);
        codegen::print_function(func);
        print!("\n");
        stdout.flush().unwrap();
    }
}
