use crate::tokens::Token;

pub struct Lexer {
    input: String,
    cur_char: Option<char>,
    cur_pos: usize,
    next_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self { input, cur_char: None, cur_pos: 0, next_pos: 0 };
        lexer.next_char();
        lexer
    }

    pub fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.cur_char {
            Some(ch) => match ch {
                c if c.is_numeric() => {
                    let first_pos = self.cur_pos;
                    while self.cur_char.is_some() && self.cur_char.unwrap().is_numeric() {
                        self.next_char();
                    }
                    let string: String = self.input[first_pos..self.cur_pos].into();
                    Token::Integer(string.parse().unwrap())
                },
                _ => todo!()
            },
            None => todo!(),
        };
        tok
    }

    fn next_char(&mut self) {
        self.cur_pos = self.next_pos;
        self.cur_char = self.input.chars().nth(self.cur_pos);
        self.next_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        match self.cur_char {
            Some(mut ch) => while ch.is_whitespace() {
                self.next_char();
                match self.cur_char {
                    Some(cch) => ch = cch,
                    None => return,
                }
            },
            None => return,
        }
    }
}