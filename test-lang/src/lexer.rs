use std::{error::Error, fmt::Display};

use crate::tokens::Token;

pub struct Lexer {
    stream: Vec<char>,

    cur_char: char,
    next_char: Option<char>,

    cur_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Result<Self, LexerError> {
        let chars: Vec<char> = input.chars().collect();
        Ok(Self {
            cur_char: match chars.get(0) {
                Some(ch) => *ch,
                None => return Err(LexerError("Invalid input string. Most likely due to being empty")),
            },
            next_char: match chars.get(1) {
                Some(ch) => Some(*ch),
                None => None,
            },
            cur_pos: 1,
            stream: chars,
        })
    }

    fn next_char(&mut self) -> Result<(), LexerError> {
        self.cur_char = match self.next_char {
            Some(ch) => ch,
            None => return Err(LexerError("Failed to get next token. End of file")),
        };
        self.cur_pos += 1;
        self.next_char = match self.stream.get(self.cur_pos) {
            Some(ch) => Some(*ch),
            None => None,
        };
        return Ok(());
    }

    pub fn tokenize(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        match self.cur_char {
            c if c.is_numeric() => {
                let mut num = String::new();
                while self.cur_char.is_numeric() {
                    num.push(self.cur_char);
                    match self.next_char() {
                        Ok(_) => (),
                        Err(_) => return Ok(Token::Integer(num.parse().expect("Failed to parse string"))),
                    }
                }
                return Ok(Token::Integer(num.parse().expect("Failed to parse string")))
            }
            _ => (),
        }
        return Err(LexerError("No token found"));
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char.is_whitespace() || self.cur_char == '\n' {
            match self.next_char() {
                Ok(_) => (),
                Err(_) => break,
            }
        }
    }

    fn debug_cur_char(&self) {
        dbg!("Current char: {}", self.cur_char);
    }
}

#[derive(Debug)]
pub struct LexerError(&'static str);

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
