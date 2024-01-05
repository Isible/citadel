use core::panic;

use clutils::{errors::FileHandlerError, files::FileHandler};

use crate::tokens::Token;

pub(crate) struct Lexer {
    pub(crate) file_handler: FileHandler,

    next_pos: usize,
    ch: Option<char>,
}

impl Lexer {
    pub(crate) fn new(path: &String) -> Result<Self, FileHandlerError> {
        let mut lexer = Self {
            file_handler: FileHandler::new(path)?,
            next_pos: 0,
            ch: None,
        };
        lexer.next_char();
        Ok(lexer)
    }

    pub(crate) fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            Some(ch) => match ch {
                c if c.is_numeric() => self.tokenize_num(),
                c if c.is_alphabetic() => self.tokenize_ident(),
                _ => self.tokenize_special_char(),
            },
            None => Token::Eof,
        }
    }

    fn tokenize_num(&mut self) -> Token {
        let mut found_fp = false;
        let first_pos = self.next_pos;
        while self.ch.is_some() && (self.ch.unwrap().is_numeric() || self.ch.unwrap() == '.') {
            if self.ch.unwrap() == '.' && !found_fp {
                found_fp = true;
            } else if self.ch.unwrap() == '.' && found_fp {
                panic!("Found a second floating point");
            }
            self.next_char();
        }

        let val = &self.file_handler.content[first_pos - 1..self.next_pos - 1];

        Token::RawLit(match found_fp {
            true => crate::tokens::Literal::Float(match val.parse() {
                Ok(val) => val,
                Err(err) => panic!("{}", err),
            }),
            false => crate::tokens::Literal::Integer(match val.parse() {
                Ok(val) => val,
                Err(err) => panic!("{}", err),
            }),
        })
    }

    fn tokenize_ident(&mut self) -> Token {
        match self.ch {
            c if c == Some('l')
                && self.file_handler.content.chars().nth(self.next_pos) == Some('{') =>
            {
                self.next_char();
                self.next_char();
                let val = self.tokenize();
                let lit = match val {
                    Token::RawLit(lit) => lit,
                    _ => panic!("Given token is not a literal even though it is in a l{{...}}"),
                };
                self.next_char();
                Token::Lit(lit)
            }
            _ => {
                let first_pos = self.next_pos;
                let mut next_char = self.file_handler.content.chars().nth(self.next_pos);
                while next_char.is_some()
                    && (next_char.unwrap().is_alphanumeric() || next_char.unwrap() == '_')
                {
                    self.next_char();
                    next_char = self.file_handler.content.chars().nth(self.next_pos);
                }
                let ident = &self.file_handler.content[first_pos - 1..self.next_pos];
                match ident {
                    "true" | "false" => {
                        Token::RawLit(crate::tokens::Literal::Boolean(match ident {
                            "true" => true,
                            _ => false,
                        }))
                    }
                    "call" => Token::Call,
                    "ret" => Token::Ret,
                    "lcl" => Token::Lcl,
                    "pub" => Token::Pub,
                    "abst" => Token::Abst,
                    "add" => Token::Add,
                    "sub" => Token::Sub,
                    "mul" => Token::Mul,
                    "div" => Token::Div,
                    _ => Token::Ident(ident.into()),
                }
            }
        }
    }

    fn tokenize_special_char(&mut self) -> Token {
        match self.ch {
            c if c == Some('"') => self.tokenize_string(),
            c if c == Some('\'') => {
                if self.file_handler.content.chars().nth(self.next_pos + 1) == Some('\'') {
                    return self.tokenize_char();
                }
                return Token::Apostrophe;
            }
            c if c == Some('=') => Token::Assign,
            c if c == Some('@') => Token::At,
            c if c == Some('%') => Token::PercentSign,
            c if c == Some('?') => Token::QuestionMark,
            c if c == Some('$') => Token::DollarSign,
            c if c == Some(':') => Token::Colon,
            c if c == Some('.') => Token::Dot,
            c if c == Some('(') => Token::LParent,
            c if c == Some(')') => Token::RParent,
            c if c == Some('[') => Token::LSquare,
            c if c == Some(']') => Token::RSquare,
            c if c == Some('{') => Token::LCurly,
            c if c == Some('}') => Token::RCurly,
            c if c == Some('#') => self.tokenize_comment(),
            _ => todo!("{:?}", self.ch),
        }
    }

    fn tokenize_char(&mut self) -> Token {
        self.next_char();
        Token::RawLit(crate::tokens::Literal::Char(match self.ch {
            Some(ch) => ch,
            None => return Token::Eof,
        }))
    }

    fn tokenize_string(&mut self) -> Token {
        let first_pos = self.next_pos;
        while self.file_handler.content.chars().nth(self.next_pos) != Some('"') {
            self.next_char();
        }
        let string = &self.file_handler.content[first_pos..self.next_pos];
        dbg!("string: {}", string);
        Token::RawLit(crate::tokens::Literal::String(string.into()))
    }

    fn tokenize_comment(&mut self) -> Token {
        self.next_char();
        while let Some(cur_ch) = self.ch {
            if cur_ch != '\n' && cur_ch != '#' {
                self.next_char();
            } else {
                break;
            }
        }
        self.next_char();
        self.tokenize()
    }

    pub(crate) fn next_char(&mut self) {
        self.ch = self.file_handler.content.chars().nth(self.next_pos);
        self.next_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        match self.ch {
            Some(mut ch) => {
                while ch.is_whitespace() {
                    self.next_char();
                    match self.ch {
                        Some(cch) => ch = cch,
                        None => return,
                    }
                }
            }
            None => return,
        }
    }
}
