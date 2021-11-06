mod lexer;
use lexer::TokenReader;

mod parser;
use parser::Parser;

fn main() {
    let source = r#"# Compute the x'th fibonacci number.
1 + func() * 3
"#;

    let token_reader = TokenReader::new(source.chars());
    let mut parser = Parser::new(token_reader);

    let expr = parser.parse_expression();
    println!("{:?}", expr);
}
