use clutils::{files::FileHandler, errors::FileHandlerError};

use crate::tokens::Token;

pub(crate) struct Lexer {
    pub(crate) file_handler: FileHandler,
    
    cur_pos: usize,
    ch: Option<char>,
}

impl Lexer {
    pub(crate) fn new(path: &String) -> Result<Self, FileHandlerError> {
        let mut lexer = Self {
            file_handler: FileHandler::new(path)?,
            cur_pos: 0,
            ch: None,
        };
        lexer.next_char();
        Ok(lexer)
    }

    pub(crate) fn tokenize(&mut self) -> Token {
        todo!()
    }

    fn next_char(&mut self) {
        self.ch = self.file_handler.content.chars().nth(self.cur_pos);
        self.cur_pos += 1;
    }
}