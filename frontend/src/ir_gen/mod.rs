/*
 * The initial to IR Converter
 */

use clutils::literal::LiteralString;

use crate::tokens::Token;

pub struct IRGenerator {
    // TODO: Might wanna make this a VecDeque
    // for easier optimizing
    stream: Vec<Token>,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            stream: Vec::new(),
        }
    }

    pub fn gen_ir(&mut self, token: Token) {
        self.stream.push(token);
    }

    pub fn get_stream(&self) -> &Vec<Token> {
        &self.stream
    }

    pub fn as_string(&self) -> String {
        let mut lit_stream = Vec::new();
        self.stream.iter().for_each(|token| lit_stream.push(token.literal()));
        lit_stream.join(" ")
    }
}
