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

    pub fn tokenize(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let tok = match self.cur_char {
            Some(ch) => match ch {
                c if c.is_numeric() => self.tokenize_int(),
                c if c.is_alphabetic() || c == '_' => self.tokenize_ident(),
                _ => return self.tokenize_symbol(),
            },
            None => Token::Eof,
        };
        Some(tok)
    }

    fn tokenize_int(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_numeric() {
            self.next_char();
        }
        let string: String = self.input[first_pos..self.cur_pos].into();
        Token::Integer(string.parse().unwrap())
    }

    fn tokenize_symbol(&mut self) -> Option<Token> {
        let ret = Some(match self.cur_char {
            Some(ch) => match ch {
                '=' => match self.input.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Equals
                    }
                    _ => Token::Assign,
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
                }
                ':' => Token::Colon,
                ',' => Token::Comma,
                '#' => {
                    self.next_char();
                    while let Some(cur_ch) = self.cur_char {
                        if cur_ch != '\n' && cur_ch != '#' {
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    self.next_char();
                    return None;
                }
                _ => panic!("Invalid symbol: {:?}", &self.cur_char),
            },
            None => todo!(),
        });
        self.next_char();
        ret
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some()
            && (self.cur_char.unwrap().is_alphanumeric() || self.cur_char.unwrap() == '_')
        {
            self.next_char();
        }
        let string: String = self.input[first_pos..self.cur_pos].into();
        return match string.as_str() {
            "let" => Token::Let,
            "loop" => Token::Loop,
            "fn" => Token::Fn,
            "if" => Token::If,
            "return" => Token::Return,
            "type" => Token::Type,
            "use" => Token::Use,

            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),

            _ => Token::Ident(self.input[first_pos..self.cur_pos].into()),
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
