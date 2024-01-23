use std::{error::Error, fmt::Display, mem::swap};

use crate::{
    ast::{
        BlockStatement, CallExpression, Expression, FnStatement, IfStatement, LetStatement,
        Literal, ReturnStatement, Statement, TypedIdent,
    },
    lexer::Lexer,
    tokens::Token,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

#[repr(u8)]
pub enum Precedence {
    Lowest,
    Assign,
    Equals,
    LG,
    LGEq,
    Sum,
    Prod,
    Prefix,
    Call,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = lexer.tokenize();
        let peek_tok = lexer.tokenize();

        Self {
            lexer,
            cur_tok,
            peek_tok,
        }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = self.lexer.tokenize();
    }

    // Every parse function needs to set cur_token to the last character in the line
    pub fn parse_stmt(&mut self) -> Result<Statement, EofError> {
        return Ok(match &self.cur_tok {
            Token::Let => Statement::Let(self.parse_let_stmt()),
            Token::Fn => Statement::Fn(self.parse_fn_stmt()),
            Token::If => Statement::If(self.parse_if_stmt()),
            Token::Type => todo!(),
            Token::Use => todo!(),
            Token::Loop => todo!(),
            Token::Return => Statement::Return(self.parse_return_stmt()),
            Token::Ident(_) => Statement::Call(self.parse_call_expr()),
            Token::Integer(_) => todo!("{}", self.cur_tok),
            Token::Float(_) => unimplemented!(),
            Token::String(_) => unimplemented!(),
            Token::Boolean(_) => unimplemented!(),
            Token::Vector(_) => unimplemented!(),
            Token::Plus => unimplemented!(),
            Token::Minus => unimplemented!(),
            Token::Divide => unimplemented!(),
            Token::Multiply => unimplemented!(),
            Token::Assign => unimplemented!(),
            Token::Semicolon => unimplemented!(),
            Token::Equals => unimplemented!(),
            Token::LParent => unimplemented!(),
            Token::RParent => todo!("next tok: {}", &self.peek_tok),
            Token::LCurly => Statement::Block(self.parse_block_stmt(Token::RCurly)),
            Token::RCurly => todo!("next tok: {}", self.peek_tok),
            Token::Colon => unimplemented!(),
            Token::Comma => unimplemented!(),
            Token::Comment(_) => {
                // skip comments and just parse the next token
                // TODO: copy ciri method for this
                self.next_token();
                self.parse_stmt()?
            }
            Token::Eof => return Err(EofError),
            Token::IntegerType(_) => todo!(),
            Token::FloatType(_) => todo!(),
        });
    }

    fn parse_expr(&mut self) -> Expression {
        match &self.cur_tok {
            Token::Ident(ident) => match self.peek_tok {
                Token::LParent => Expression::Call(self.parse_call_expr()),
                _ => Expression::Literal(Literal::Variable(ident.into())),
            },
            Token::Integer(int) => Expression::Literal(Literal::Integer(*int)),
            Token::Float(float) => Expression::Literal(Literal::Float(*float)),
            Token::String(string) => Expression::Literal(Literal::String(string.into())),
            Token::Boolean(boolean) => Expression::Literal(Literal::Boolean(*boolean)),
            _ => todo!("cur: {:?}, next: {:?}", self.cur_tok, self.peek_tok),
        }
    }

    fn parse_let_stmt(&mut self) -> LetStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.to_string()));
        let name = self.parse_typed_ident();
        self.expect_peek_tok(Token::Assign);
        // skip name and assign
        self.next_token();
        self.next_token();

        let val = self.parse_expr();
        self.expect_peek_tok(Token::Semicolon);

        // expression value and semicolon
        self.next_token();
        LetStatement { name, val }
    }

    fn parse_fn_stmt(&mut self) -> FnStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.to_string()));
        self.next_token();

        let name = self.cur_tok.to_string();

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

        self.expect_peek_tok(Token::Ident(self.peek_tok.to_string()));
        self.next_token();

        let ret_type = self.cur_tok.to_string();

        self.expect_peek_tok(Token::LCurly);
        self.next_token();

        let block = self.parse_block_stmt(Token::RCurly);

        // parse_block_stmt(...) sets the current token to the end token
        // so we don't have to do anything here

        FnStatement {
            name,
            args,
            ret_type,
            block,
        }
    }

    fn parse_if_stmt(&mut self) -> IfStatement {
        self.next_token();
        let condition = self.parse_expr();
        self.expect_peek_tok(Token::LCurly);
        // go to left curly bracket
        self.next_token();
        let block = self.parse_block_stmt(Token::RCurly);

        self.next_token();

        IfStatement { condition, block }
    }

    fn parse_return_stmt(&mut self) -> ReturnStatement {
        self.next_token();
        let val = self.parse_expr();
        self.expect_peek_tok(Token::Semicolon);
        self.next_token();
        ReturnStatement { val }
    }

    /// cur token should be the beginning of the block, for example: `{`
    /// sets cur token to the end token (function argument)
    fn parse_block_stmt(&mut self, end: Token) -> BlockStatement {
        let mut block = Vec::new();
        self.next_token();
        while self.cur_tok != end {
            block.push(match self.parse_stmt() {
                Ok(stmt) => stmt,
                Err(_) => break,
            });
            self.next_token();
        }

        BlockStatement { stmts: block }
    }

    /// Parses the arguments of a function definition
    ///
    /// cur_token should be beginning of the list, for example `(`
    fn parse_def_args(&mut self) -> Vec<TypedIdent> {
        let mut args = Vec::new();
        loop {
            args.push(self.parse_typed_ident());
            if self.peek_tok == Token::Comma {
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
        // go to ident
        self.expect_peek_tok(Token::Ident(self.peek_tok.to_string()));
        self.next_token();
        let ident = self.cur_tok.to_string();
        // go to colon
        self.expect_peek_tok(Token::Colon);
        self.next_token();

        self.expect_peek_tok_as_type();
        // go to next ident
        self.next_token();
        let _type = self.cur_tok.to_string();

        TypedIdent { ident, _type }
    }

    fn parse_call_expr(&mut self) -> CallExpression {
        let name = self.cur_tok.to_string();

        self.expect_peek_tok(Token::LParent);

        self.next_token();

        let args = self.parse_call_args();

        if self.peek_tok == Token::Semicolon {
            self.next_token();
        }

        CallExpression { name, args }
    }

    // cur token is a Left parenthesis
    fn parse_call_args(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();

        if self.peek_tok == Token::RParent {
            self.next_token();
            return args;
        }

        self.next_token();

        args.push(self.parse_expr());

        while self.peek_tok == Token::Comma {
            self.next_token();
            self.next_token();
            args.push(self.parse_expr());
        }

        self.next_token();

        args
    }

    fn expect_peek_tok_as_type(&self) {
        match self.peek_tok {
            Token::IntegerType(_) => (),
            Token::FloatType(_) => (),
            Token::Ident(_) => (),
            Token::Vector(_) => (),
            _ => panic!("expected a type, got: {}", self.peek_tok),
        }
    }

    fn expect_peek_tok(&self, expect: Token) {
        if self.peek_tok != expect {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }
}

#[derive(Debug)]
pub struct EofError;

impl Error for EofError {}

impl Display for EofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Encountered end of file")
    }
}
