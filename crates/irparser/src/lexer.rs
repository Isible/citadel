use logos::Logos;

use crate::tokens::Token;

pub struct Lexer<'l> {
    pub tokens: Vec<Token<'l>>,
    pub source: &'l str,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        let tokens = Token::lexer(source)
            .map(|tok| match tok {
                Ok(tok) => tok,
                Err(()) => panic!(),
            })
            .collect();
        Self { tokens, source }
    }
}
