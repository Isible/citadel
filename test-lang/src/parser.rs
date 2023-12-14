use std::mem::swap;

use clutils::{error::{throw, Error}, literal::LiteralString};

use crate::{lexer::Lexer, tokens::Token, ast::{Statement, LetStatement, Expression, Literal, Ident}};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,

    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = lexer.tokenize();
        let peek_tok = lexer.tokenize();

        Self {
            lexer,
            cur_tok,
            peek_tok,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = self.lexer.tokenize();
    }

    pub fn parse_stmt(&mut self) -> Statement {
        return match &self.cur_tok {
            Token::Let => self.parse_let_stmt(),
            Token::Fn => todo!(),
            Token::If => todo!(),
            Token::Loop => todo!(),
            Token::Ident(_) => todo!(),
            Token::Integer(_) => todo!(),
            Token::Float(_) => todo!(),
            Token::String(_) => todo!(),
            Token::Boolean(_) => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::Divide => todo!(),
            Token::Multiply => todo!(),
            Token::Assign => todo!(),
            Token::Semicolon => todo!(),
            Token::Equals => todo!(),
            Token::LParent => todo!(),
            Token::RParent => todo!(),
            Token::LCurly => todo!(),
            Token::RCurly => todo!(),
            Token::Eof => todo!(),
        };
    }

    fn parse_expr(&self) -> Expression {
        match &self.cur_tok {
            Token::Ident(ident) => Expression::Literal(Literal::Ident(ident.into())),
            Token::Integer(int) => Expression::Literal(Literal::Integer(*int)),
            Token::Float(float) => Expression::Literal(Literal::Float(*float)),
            Token::String(string) => Expression::Literal(Literal::String(string.into())),
            Token::Boolean(boolean) => Expression::Literal(Literal::Boolean(*boolean)),
            _ => todo!("{:?}", self.cur_tok)
        }
    }

    fn parse_let_stmt(&mut self) -> Statement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();
        let name = self.cur_tok.literal();
        self.expect_peek_tok(Token::Assign);
        // skip name and assign
        self.next_token();
        self.next_token();

        let val = self.parse_expr();
        Statement::Let(LetStatement { name: Ident(name), val })
    }

    fn expect_peek_tok(&self, expect: Token) {
        if *self.peek_tok.literal() != expect.literal() {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }
}

struct InvalidTok<'a> {
    expected: Token,
    received: &'a Token,
}

impl Error for InvalidTok<'_> {
    fn name(&self) -> &str {
        "Invalid Token"
    }

    fn desc(&self) -> String {
        format!("expected: {:?}, received: {:?}", self.expected.literal(), self.received.literal())
    }

    fn additional_ctx(&self) -> Option<Vec<String>> {
        None
    }

    fn tip(&self) -> Option<String> {
        None
    }
}
