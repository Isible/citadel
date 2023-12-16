use core::arch;
use std::{borrow::BorrowMut, fmt::Formatter, io::SeekFrom, mem::swap};

use clutils::{
    error::{throw, Error},
    literal::LiteralString,
};

use crate::{
    ast::{
        BlockStatement, Expression, FnStatement, Ident, LetStatement, Literal, Statement,
        TypedIdent,
    },
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
            Token::Ident(ident) => todo!("{}", ident),
            Token::Integer(_) => todo!(),
            Token::Float(_) => todo!(),
            Token::String(_) => todo!(),
            Token::Boolean(_) => todo!(),
            Token::Vector(_) => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::Divide => todo!(),
            Token::Multiply => todo!(),
            Token::Assign => todo!(),
            Token::Semicolon => todo!(),
            Token::Equals => todo!(),
            Token::LParent => todo!(),
            Token::RParent => todo!(),
            Token::LCurly => Statement::Block(self.parse_block_stmt(Token::RCurly)),
            Token::RCurly => todo!(),
            Token::Colon => todo!(),
            Token::Comma => todo!(),
            Token::Comment(_) => {
                self.next_token();
                self.parse_stmt()
            }
            Token::Eof => panic!("Encountered End of file"),
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
        let name = self.parse_typed_ident();
        self.expect_peek_tok(Token::Assign);
        // skip name and assign
        self.next_token();
        self.next_token();

        let val = self.parse_expr();
        self.expect_peek_tok(Token::Semicolon);

        // expression value and semicolon
        self.next_token();
        self.next_token();
        LetStatement { name, val }
    }

    fn parse_fn_stmt(&mut self) -> FnStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();

        let name = Ident(self.cur_tok.literal());

        self.expect_peek_tok(Token::LParent);
        self.next_token();

        let args = match self.peek_tok {
            Token::RParent => {
                self.next_token();
                Vec::new()
            }
            _ => self.parse_def_args(),
        };

        self.expect_peek_tok(Token::Colon);
        self.next_token();
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();

        let ret_type = Ident(self.cur_tok.literal());

        self.expect_peek_tok(Token::LCurly);
        self.next_token();
        let block = self.parse_block_stmt(Token::RCurly);
        FnStatement {
            name,
            args,
            ret_type,
            block,
        }
    }

    /// cur token should be the beginning of the block, for example: `{`
    fn parse_block_stmt(&mut self, end: Token) -> BlockStatement {
        let mut block = Vec::new();
        self.next_token();
        while self.cur_tok != end {
            block.push(self.parse_stmt());
        }

        self.next_token();

        BlockStatement { stmts: block }
    }

    /// Parses the arguments of a function definition
    ///
    /// cur_token should be beginning of the list, for example `(`
    fn parse_def_args(&mut self) -> Vec<TypedIdent> {
        let mut args = Vec::new();
        self.next_token();
        self.print_cur_tok();
        loop {
            args.push(self.parse_typed_ident());
            if self.peek_tok == Token::Comma {
                self.next_token();
                self.next_token();
            } else if self.cur_tok == Token::RParent || self.peek_tok == Token::RParent {
                break;
            } else {
                self.expect_peek_tok(Token::RParent);
            }
        }
        self.next_token();

        args
    }

    /// cur_token should be the token before the first TypedIdent
    ///
    /// cur_token gets set to the type of the ident
    fn parse_typed_ident(&mut self) -> TypedIdent {
        self.print_cur_tok();
        // go to ident
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        let ident = Ident(self.cur_tok.literal());
        dbg!("ident: {}", &ident);
        // go to colon
        self.expect_peek_tok(Token::Colon);
        self.next_token();
        // go to next ident
        self.expect_peek_tok(Token::Ident(self.peek_tok.literal()));
        self.next_token();
        let _type = Ident(self.cur_tok.literal());

        TypedIdent { ident, _type }
    }

    fn expect_peek_tok(&self, expect: Token) {
        if *self.peek_tok.literal() != expect.literal() {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }

    fn print_cur_tok(&self) {
        dbg!("{}", &self.cur_tok);
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
