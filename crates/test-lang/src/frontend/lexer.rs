//! Lexer for the tokenizing source code into tokens.

use logos::Logos;

use super::tokens::Token;

pub struct Lexer<'l> {
    pub input: &'l str,
    pub tokens: Vec<Token<'l>>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l str) -> Self {
        let mut tokens = Vec::new();
        for token in Token::lexer(input) {
            tokens.push(token.unwrap())
        }
        Self { input, tokens }
    }
}
