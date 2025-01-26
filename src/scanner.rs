use std::cell::LazyCell;
use std::collections::HashMap;

use crate::errors::Error;
use crate::tokens::{Literal, Token, TokenType};
use anyhow::bail;

const KEYWORDS: LazyCell<HashMap<&str, TokenType>> = LazyCell::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

#[derive(Default)]
pub struct Scanner {
    tokens: Vec<Token>,
    source: Vec<u8>,
    error: bool,
    start: usize,
    current: usize,
    line: u64,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source: source.into_bytes(),
            line: 1,
            ..Default::default()
        }
    }

    fn done(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    fn scan(&mut self) {
        //for char in self.source.chars() {}
        while !self.done() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                // TODO handle error
                self.error = true;
            };
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".into(),
            line: self.line,
            literal: None,
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        char::from(self.source[self.current - 1])
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.done() {
            return false;
        }
        if char::from(self.source[self.current]) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.done() {
            return '\0';
        }
        char::from(self.source[self.current])
    }

    fn string(&mut self) -> anyhow::Result<()> {
        while self.peek() != '"' && !self.done() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.done() {
            bail!(Error::ParseError("unterminated string".to_string()));
        }

        self.current += 1;
        let value = self.source[self.start + 1..self.current - 1].to_vec();
        let parsed = String::from_utf8(value.clone())?;
        self.add_token_value(TokenType::String, value, Some(Literal::String(parsed)));

        Ok(())
    }

    fn number(&mut self) -> anyhow::Result<()> {
        while Self::is_digit(self.peek()) {
            self.current += 1;
        }

        if self.peek() == '.' && Self::is_digit(self.source[self.current + 1] as char) {
            self.current += 1;
            while Self::is_digit(self.peek()) {
                self.current += 1;
            }
        }

        let value = self.source[self.start..self.current].to_vec();
        let parsed = String::from_utf8(value.clone())?;
        let parsed = parsed.parse::<f64>()?;
        self.add_token_value(TokenType::Number, value, Some(Literal::Number(parsed)));
        Ok(())
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn identifier(&mut self) -> anyhow::Result<()> {
        while Self::is_alphanumeric(self.peek()) {
            self.current += 1;
        }

        let value = self.source[self.start..self.current].to_vec();
        let parsed = String::from_utf8(value.clone())?;
        if let Some(token_type) = KEYWORDS.get(parsed.as_str()) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::Identifier);
        }

        Ok(())
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBrace),
            ']' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.done() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' => self.string()?,

            // unrecognized
            _ => {
                if Self::is_digit(c) {
                    self.number()?;
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    self.error = true;
                    bail!(Error::ParseError(format!("unrecognized character '{c}'")));
                }
            }
        };
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            line: self.line,
            lexeme: "".into(),
            literal: None,
        });
    }

    fn add_token_value(
        &mut self,
        token_type: TokenType,
        lexeme: Vec<u8>,
        literal: Option<Literal>,
    ) {
        self.tokens.push(Token {
            token_type,
            line: self.line,
            lexeme,
            literal,
        });
    }

    fn is_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::new("1 + 2".to_string());
        scanner.scan();
        assert_eq!(scanner.tokens.len(), 4);
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_scanner_string() {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        scanner.scan();
        assert_eq!(scanner.tokens.len(), 2);
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_scanner_string_error() {
        let mut scanner = Scanner::new("\"hello".to_string());
        scanner.scan();
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.error, true);
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_scanner_number() {
        let mut scanner = Scanner::new("1.0".to_string());
        scanner.scan();
        assert_eq!(scanner.tokens.len(), 2);
        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_scanner_multiline() {
        let mut scanner = Scanner::new("1 + 2\n3 + 4".to_string());
        scanner.scan();
        assert_eq!(scanner.tokens.len(), 7);
        println!("{:?}", scanner.tokens);
    }
}
