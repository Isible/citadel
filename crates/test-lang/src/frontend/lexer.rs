//! Lexer for the tokenizing source code into tokens.

use logos::Logos;

use super::tokens::Token;

#[derive(Default)]
pub struct Lexer<'l> {
    pub tokens: Vec<Token<'l>>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l str) -> Self {
        let mut tokens = Vec::new();
        for token in Token::lexer(input) {
            tokens.push(match token.unwrap() {
                Token::LitString(str) => Token::LitString(Self::trim_marks(str)),
                Token::LitChar(str) => Token::LitChar(Self::trim_marks(str)),
                tok => tok,
            })
        }
        Self { tokens }
    }

    fn trim_marks(input: &str) -> &str {
        &input[1..input.len() - 1]
    }
}
