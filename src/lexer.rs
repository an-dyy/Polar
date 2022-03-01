//! Polar LEXER.
//! This file handles lexical analysis.

use crate::token::{Token, TokenKind};

#[allow(dead_code)]
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        return Self {
            source: source,
            index: 0,
        };
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for token in self.source.chars() {
            tokens.push(
                Token::new(token.to_string(),
                TokenKind::Invalid)
            );
        }

        return tokens;
    }
}
