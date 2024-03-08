//! Parser module for parsing test-lang tokens into an abstract syntax tree

use std::{error::Error, fmt::Display, mem::swap};

use crate::{
    ast::{
        BlockStatement, CallExpression, Expression, FnStatement, IfStatement, InfixOpExpr,
        LetStatement, Literal, Operator, ReturnStatement, Statement, TypedIdent,
    },
    lexer::Lexer,
    tokens::Token,
    util,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
        let cur_tok = util::get_next_tok(lexer);
        let peek_tok = util::get_next_tok(lexer);
        Self {
            lexer,
            cur_tok,
            peek_tok,
        }
    }

    // Every parse function needs to set cur_token to the last character in the line
    pub fn parse_stmt(&mut self) -> Result<Statement, EofError> {
        Ok(match &self.cur_tok {
            Token::Let => Statement::Let(self.parse_let_stmt()),
            Token::Fn => Statement::Fn(self.parse_fn_stmt()),
            Token::If => Statement::If(self.parse_if_stmt()),
            Token::Use => todo!(),
            Token::Loop => todo!(),
            Token::Return => Statement::Return(self.parse_return_stmt()),
            Token::LCurly => Statement::Block(self.parse_block_stmt(Token::RCurly)),
            Token::RCurly => todo!("next tok: {}", self.peek_tok),
            Token::Eof => return Err(EofError),
            _ => Statement::Expression(self.parse_expr(Precedence::Lowest)),
        })
    }

    fn parse_expr(&mut self, prec: Precedence) -> Expression {
        let prefix = self.parse_prefix();

        let mut left_expression = prefix;

        while !self.peek_is_end() && prec < self.get_precedence(&self.peek_tok) {
            self.next_token();
            // Unwrap here might not be safe. Observe this
            left_expression = self.parse_infix(left_expression);
        }

        left_expression
    }

    fn parse_infix(&mut self, left: Expression) -> Expression {
        match self.cur_tok {
            Token::Equals | Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                self.parse_infix_expr(left)
            }
            Token::LParent => Expression::Call(self.parse_call_expr(left)),
            // Token::LSquare => self.parse_index_expr(left),
            _ => panic!("Invalid for parsing an infix expr: {:#?}", left),
        }
    }

    fn parse_prefix(&mut self) -> Expression {
        match &self.cur_tok {
            Token::Integer(int) => Expression::Literal(Literal::Integer(*int)),
            Token::Float(float) => Expression::Literal(Literal::Float(*float)),
            Token::String(string) => Expression::Literal(Literal::String(string.into())),
            Token::Boolean(boolean) => Expression::Literal(Literal::Boolean(*boolean)),
            Token::Ident(ident) => Expression::Literal(Literal::Ident(ident.into())), 
            _ => panic!("No prefix parse found for: {}", self.cur_tok),
        }
    }

    fn parse_infix_expr(&mut self, left_expr: Expression) -> Expression {
        let op = self.cur_tok_to_in_op();
        let prec = self.get_precedence(&self.cur_tok);
        self.next_token();
        let right_expr = self.parse_expr(prec);
        Expression::Infix(InfixOpExpr {
            sides: (Box::from(left_expr), Box::from(right_expr)),
            operator: op,
        })
    }

    fn parse_let_stmt(&mut self) -> LetStatement {
        self.expect_peek_tok(Token::Ident(self.peek_tok.to_string()));
        let name = self.parse_typed_ident();
        self.expect_peek_tok(Token::Assign);
        // skip name and assign
        self.next_token();
        self.next_token();

        let val = self.parse_expr(Precedence::Lowest);
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
        let condition = self.parse_expr(Precedence::Lowest);
        self.expect_peek_tok(Token::LCurly);
        // go to left curly bracket
        self.next_token();
        let block = self.parse_block_stmt(Token::RCurly);

        self.next_token();

        IfStatement { condition, block }
    }

    fn parse_return_stmt(&mut self) -> ReturnStatement {
        self.next_token();
        let val = self.parse_expr(Precedence::Lowest);
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

    fn parse_call_expr(&mut self, left: Expression) -> CallExpression {
        let name = match left {
            Expression::Literal(Literal::Ident(var)) => var,
            _ => panic!("{left:?} is not an ident"),
        };

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

        args.push(self.parse_expr(Precedence::Lowest));

        while self.peek_tok == Token::Comma {
            self.next_token();
            self.next_token();
            args.push(self.parse_expr(Precedence::Lowest));
        }

        self.next_token();

        args
    }

    fn expect_peek_tok_as_type(&self) {
        match self.peek_tok {
            Token::IntegerType(_) => (),
            Token::FloatType(_) => (),
            Token::Ident(_) => (),
            _ => panic!("expected a type, got: {}", self.peek_tok),
        }
    }

    fn expect_peek_tok(&self, expect: Token) {
        if self.peek_tok != expect {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = util::get_next_tok(self.lexer);
    }

    fn cur_tok_to_in_op(&self) -> Operator {
        match self.cur_tok {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Sub,
            Token::Divide => Operator::Div,
            Token::Multiply => Operator::Mul,
            Token::Assign => todo!(),
            Token::Equals => Operator::Equals,
            _ => panic!("Cannot convert {} to operator", self.cur_tok),
        }
    }

    fn peek_is_end(&self) -> bool {
        matches!(self.peek_tok, Token::Semicolon | Token::Eof)
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Assign => Precedence::Assign,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Multiply | Token::Divide => Precedence::Prod,
            Token::Equals => Precedence::Equals,
            Token::LParent => Precedence::Call,
            _ => Precedence::Lowest,
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
