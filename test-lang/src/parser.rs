use std::{fmt::Formatter, mem::swap};

use clutils::{
    error::{throw, Error},
    literal::LiteralString,
};

use crate::{
    ast::{BlockStatement, Expression, FnStatement, Ident, LetStatement, Literal, Statement},
    lexer::Lexer,
    tokens::Token,
};

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

    // Every parse function needs to set cur_token to the semicolon
    pub fn parse_stmt(&mut self) -> Statement {
        return match &self.cur_tok {
            Token::Let => Statement::Let(self.parse_let_stmt()),
            Token::Fn => Statement::Fn(self.parse_fn_stmt()),
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
            _ => todo!("{:?}", self.cur_tok),
        }
    }

    fn parse_let_stmt(&mut self) -> LetStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();
        let name = self.cur_tok.literal();
        self.expect_peek_tok(Token::Assign);
        // skip name and assign
        self.next_token();
        self.next_token();

        let val = self.parse_expr();
        self.expect_peek_tok(Token::Semicolon);

        // expression value and semicolon
        self.next_token();
        self.next_token();
        LetStatement {
            name: Ident(name),
            val,
        }
    }

    fn parse_fn_stmt(&mut self) -> FnStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();
        let name = Ident(self.cur_tok.literal());
        self.expect_peek_tok(Token::LParent);
        self.next_token();
        self.expect_peek_tok(Token::RParent);
        self.next_token();
        self.expect_peek_tok(Token::LCurly);
        let block = self.parse_block_stmt(Token::RCurly);
        FnStatement {
            name,
            args: Vec::new(),
            block,
        }
    }

    /// cur token should be the beginning of thw block, for example: `{`
    fn parse_block_stmt(&mut self, end: Token) -> BlockStatement {
        let mut block = Vec::new();
        self.next_token();
        self.next_token();
        while self.cur_tok != end {
            block.push(self.parse_stmt());
        }

        self.next_token();

        BlockStatement { stmts: block }
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
        format!(
            "expected: {:?}, received: {:?}",
            self.expected.literal(),
            self.received.literal()
        )
    }

    fn additional_ctx(&self) -> Option<Vec<String>> {
        None
    }

    fn tip(&self) -> Option<String> {
        None
    }
}
