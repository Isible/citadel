use logos::Logos;

use crate::{tokens::Token, utils};

pub struct Lexer<'l> {
    pub tokens: Vec<Token<'l>>,
    pub source: &'l str,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        let tokens = Token::lexer(source)
            .map(|tok| match tok {
                Ok(tok) => match tok {
                    Token::LitChar(ch) => utils::trim_lit_char(ch),
                    Token::LitInt(int) => utils::trim_lit_int(int),
                    Token::LitString(str) => utils::trim_lit_str(str),
                    _ => tok
                },
                Err(()) => panic!(),
            })
            .collect();
        Self { tokens, source }
    }
}
