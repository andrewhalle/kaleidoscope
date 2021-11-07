use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    LessThan,
    Plus,
    Minus,
    Star,
    LParen,
    RParen,
    Comma,
    Semicolon,
    Identifier(String),
    Number(f64),
}

pub struct TokenReader<C: Iterator<Item = char>> {
    has_returned_eof: bool,
    reader: Peekable<C>,
}

impl<C: Iterator<Item = char>> Iterator for TokenReader<C> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.get_token();
        match token {
            Token::Eof => {
                if self.has_returned_eof {
                    None
                } else {
                    self.has_returned_eof = true;
                    Some(token)
                }
            }
            _ => Some(token),
        }
    }
}

impl<C: Iterator<Item = char>> TokenReader<C> {
    fn skip_whitespace_and_comments(&mut self) {
        let mut comment = self.reader.peek().map(|c| c == &'#').unwrap_or(false);

        while comment
            || self.reader.peek().map(|c| c == &'#').unwrap_or(false)
            || self
                .reader
                .peek()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
        {
            match self.reader.next() {
                None => break,
                Some(c) => {
                    if c == '\n' {
                        comment = false;
                    } else if c == '#' {
                        comment = true;
                    }
                }
            }
        }
    }

    // should have already verified that the next character starts an identifier
    fn get_identifier(&mut self) -> String {
        let mut s = String::new();

        loop {
            let next = self.reader.peek();
            match next {
                Some(c) if c.is_alphanumeric() => {
                    s.push(*c);
                    self.reader.next();
                }
                _ => break,
            }
        }

        s
    }

    // should have already verified that the next character starts an number
    fn get_number(&mut self) -> f64 {
        let mut s = String::new();

        loop {
            let next = self.reader.peek();
            match next {
                Some(c) if c.is_numeric() || c == &'.' => {
                    s.push(*c);
                    self.reader.next();
                }
                _ => break,
            }
        }

        s.parse().expect("Not a number.")
    }

    fn get_non_eof_token(&mut self) -> Option<Token> {
        self.skip_whitespace_and_comments();

        if self.reader.peek()?.is_alphabetic() {
            let identifier = self.get_identifier();

            Some(match identifier.as_str() {
                "def" => Token::Def,
                "extern" => Token::Extern,
                _ => Token::Identifier(identifier),
            })
        } else if self.reader.peek()?.is_numeric() || self.reader.peek()? == &'.' {
            Some(Token::Number(self.get_number()))
        } else {
            Some(match self.reader.next()? {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '<' => Token::LessThan,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '*' => Token::Star,
                c => panic!("Unexpected character. {}", c),
            })
        }
    }

    pub fn new(reader: C) -> Self {
        TokenReader {
            has_returned_eof: false,
            reader: reader.peekable(),
        }
    }

    pub fn get_token(&mut self) -> Token {
        match self.get_non_eof_token() {
            None => Token::Eof,
            Some(token) => token,
        }
    }
}
