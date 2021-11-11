use std::iter::Peekable;

use crate::lexer::Token;

fn precedence(token: Option<&Token>) -> Option<u8> {
    token
        .map(|token| match token {
            Token::LessThan => Some(10),
            Token::Plus => Some(20),
            Token::Minus => Some(30),
            Token::Star => Some(40),
            _ => None,
        })
        .flatten()
}

#[derive(Debug)]
pub enum ExprAstNode {
    Null,
    Number(NumberExprAstNode),
    Variable(VariableExprAstNode),
    Binary(BinaryExprAstNode),
    Call(CallExprAstNode),
}

#[derive(Debug)]
pub enum AstNode {
    Function(FunctionAstNode),
    Prototype(PrototypeAstNode),
}

#[derive(Debug)]
pub struct NumberExprAstNode {
    pub value: f64,
}

#[derive(Debug)]
pub struct VariableExprAstNode {
    pub name: String,
}

#[derive(Debug)]
pub struct BinaryExprAstNode {
    pub op: Token,
    pub lhs: Box<ExprAstNode>,
    pub rhs: Box<ExprAstNode>,
}

#[derive(Debug)]
pub struct CallExprAstNode {
    // should this be an ExprAstNode?
    callee: String,
    args: Vec<ExprAstNode>,
}

#[derive(Debug, Default)]
pub struct PrototypeAstNode {
    name: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct FunctionAstNode {
    pub prototype: PrototypeAstNode,
    pub body: Box<ExprAstNode>,
}

#[derive(Debug)]
pub struct Parser<T: Iterator<Item = Token>> {
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    fn parse_number_expr(&mut self) -> Result<ExprAstNode, ()> {
        let token = self.tokens.next();
        match token {
            Some(Token::Number(value)) => Ok(ExprAstNode::Number(NumberExprAstNode { value })),
            _ => Err(()),
        }
    }

    fn parse_paren_expr(&mut self) -> Result<ExprAstNode, ()> {
        let token = self.tokens.next();
        if token.is_some() && token.unwrap() != Token::LParen {
            return Err(());
        }

        if self.tokens.peek() == Some(&Token::RParen) {
            return Ok(ExprAstNode::Null);
        }
        let expr = self.parse_expression()?;

        let token = self.tokens.next();
        if token.is_some() && token.unwrap() != Token::RParen {
            return Err(());
        }

        Ok(expr)
    }

    fn parse_identifier_expr(&mut self) -> Result<ExprAstNode, ()> {
        let token = self.tokens.peek();
        if token.is_some() && !matches!(token, Some(Token::Identifier(_))) {
            return Err(());
        }

        let token = self.tokens.next().unwrap();
        let name = match token {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };

        // not a call
        let token = self.tokens.peek();
        if token.is_some() && !matches!(token.unwrap(), Token::LParen) {
            return Ok(ExprAstNode::Variable(VariableExprAstNode { name }));
        }

        // a call
        self.tokens.next();
        let mut args = Vec::new();
        if !matches!(self.tokens.peek(), Some(Token::RParen)) {
            loop {
                let expr = self.parse_expression()?;
                args.push(expr);

                if matches!(self.tokens.peek(), Some(Token::RParen)) {
                    break;
                }
                if !matches!(self.tokens.peek(), Some(Token::Comma)) {
                    return Err(());
                }

                // eat ','.
                self.tokens.next();
            }
        }

        // eat ')'.
        self.tokens.next();

        let callee = name;
        Ok(ExprAstNode::Call(CallExprAstNode { callee, args }))
    }

    fn parse_primary(&mut self) -> Result<ExprAstNode, ()> {
        match self.tokens.peek() {
            Some(Token::Identifier(_)) => self.parse_identifier_expr(),
            Some(Token::Number(_)) => self.parse_number_expr(),
            Some(Token::LParen) => self.parse_paren_expr(),
            _ => Err(()),
        }
    }

    fn parse_bin_op_rhs(
        &mut self,
        min_precedence: u8,
        mut lhs: ExprAstNode,
    ) -> Result<ExprAstNode, ()> {
        loop {
            let token_precedence = precedence(self.tokens.peek()).unwrap_or(0);
            if precedence(self.tokens.peek()).is_none() || token_precedence < min_precedence {
                return Ok(lhs);
            }

            // we have a binary op
            let op = self.tokens.next().unwrap();
            let mut rhs = self.parse_primary()?;

            let next_precedence = precedence(self.tokens.peek()).unwrap_or(0);
            if precedence(Some(&op)).unwrap() < next_precedence {
                rhs = self.parse_bin_op_rhs(token_precedence, rhs)?;
            }

            lhs = ExprAstNode::Binary(BinaryExprAstNode {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn parse_prototype(&mut self) -> Result<PrototypeAstNode, ()> {
        if !matches!(self.tokens.peek(), Some(Token::Identifier(_))) {
            return Err(());
        }

        let name = match self.tokens.next().unwrap() {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };

        if !matches!(self.tokens.peek(), Some(Token::LParen)) {
            return Err(());
        }
        self.tokens.next();

        let mut args = Vec::new();
        while let Some(Token::Identifier(_)) = self.tokens.peek() {
            let name = match self.tokens.next().unwrap() {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };

            args.push(name);
        }

        if !matches!(self.tokens.peek(), Some(Token::RParen)) {
            return Err(());
        }
        self.tokens.next();

        Ok(PrototypeAstNode { name, args })
    }

    fn parse_definition(&mut self) -> Result<FunctionAstNode, ()> {
        if !matches!(self.tokens.peek(), Some(Token::Def)) {
            return Err(());
        }
        self.tokens.next();

        let prototype = self.parse_prototype()?;
        let body = Box::new(self.parse_expression()?);

        Ok(FunctionAstNode { prototype, body })
    }

    fn parse_extern(&mut self) -> Result<PrototypeAstNode, ()> {
        if !matches!(self.tokens.peek(), Some(Token::Extern)) {
            return Err(());
        }
        self.tokens.next();

        self.parse_prototype()
    }

    fn parse_top_level_expr(&mut self) -> Result<AstNode, ()> {
        let body = Box::new(self.parse_expression()?);

        Ok(AstNode::Function(FunctionAstNode {
            prototype: Default::default(),
            body,
        }))
    }

    pub fn parse_expression(&mut self) -> Result<ExprAstNode, ()> {
        let lhs = self.parse_primary()?;

        self.parse_bin_op_rhs(0, lhs)
    }

    pub fn parse_top_level(&mut self) -> Option<AstNode> {
        match self.tokens.peek() {
            Some(Token::Eof) => None,
            Some(Token::Semicolon) => None,
            Some(Token::Def) => Some(AstNode::Function(self.parse_definition().unwrap())),
            Some(Token::Extern) => Some(AstNode::Prototype(self.parse_extern().unwrap())),
            _ => Some(self.parse_top_level_expr().unwrap()),
        }
    }

    pub fn new(tokens: T) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }
}
