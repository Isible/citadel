use crate::tokens::Token;

pub struct Lexer {
    input: String,
    cur_char: Option<char>,
    cur_pos: usize,
    next_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            cur_char: None,
            cur_pos: 0,
            next_pos: 0,
        };
        lexer.next_char();
        lexer
    }

    pub fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.cur_char {
            Some(ch) => match ch {
                c if c.is_numeric() => self.tokenize_int(),
                c if c.is_alphabetic() => self.tokenize_ident(),
                _ => self.tokenize_symbol(),
            },
            None => Token::Eof,
        };
        tok
    }

    fn tokenize_int(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_numeric() {
            self.next_char();
        }
        let string: String = self.input[first_pos..self.cur_pos].into();
        Token::Integer(string.parse().unwrap())
    }

    fn tokenize_symbol(&mut self) -> Token {
        let ret = match self.cur_char {
            Some(ch) => match ch {
                '=' => match self.input.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Equals
                    },
                    _ => Token::Assign
                },
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                ';' => Token::Semicolon,
                '(' => Token::LParent,
                ')' => Token::RParent,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                '"' => {
                    self.next_char();
                    let first_pos = self.cur_pos;
                    while self.cur_char != Some('"') {
                        self.next_char();
                    }
                    let string: String = self.input[first_pos..self.cur_pos].into();
                    Token::String(string)
                },
                _ => panic!("Invalid symbol"),
            },
            None => todo!(),
        };
        self.next_char();
        ret
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_alphanumeric() {
            self.next_char();
        }
        let string: String = self.input[first_pos..self.cur_pos].into();
        return match string.as_str() {
            "let" => Token::Let,
            "loop" => Token::Loop,
            "fn" => Token::Fn,
            "if" => Token::If,
            _ => Token::Ident(string),
        };
    }

    fn next_char(&mut self) {
        self.cur_pos = self.next_pos;
        self.cur_char = self.input.chars().nth(self.cur_pos);
        self.next_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        match self.cur_char {
            Some(mut ch) => {
                while ch.is_whitespace() {
                    self.next_char();
                    match self.cur_char {
                        Some(cch) => ch = cch,
                        None => return,
                    }
                }
            }
            None => return,
        }
    }
}
