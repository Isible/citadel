//! Parser module for parsing test-lang tokens into an abstract syntax tree

use bumpalo::Bump;

use crate::{expect_tok, frontend::ast::{LoopStatement, Type}};

use super::{
    ast::{
        BlockStatement, CallExpression, Expression, FnStatement, IfStatement, InfixOpExpr,
        LetStatement, Literal, Operator, ReturnStatement, Statement, TypedIdent,
    },
    lexer::Lexer,
    tokens::Token,
};

pub struct Parser<'p> {
    lexer: &'p Lexer<'p>,
    arena: &'p Bump,

    tok_index: usize,
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

impl<'p> Parser<'p> {
    pub fn new(lexer: &'p Lexer, arena: &'p Bump) -> Self {
        Self {
            lexer,
            arena,
            tok_index: 0,
        }
    }

    pub fn parse_program(&mut self) -> Vec<Statement<'p>> {
        let mut program = Vec::new();
        while let Some(stmt) = self.parse_stmt() {
            dbg!(&stmt);
            program.push(stmt);
            self.next_tok();
        }
        program
    }

    // Every parse function needs to set cur_token to the last character in the line
    pub fn parse_stmt(&mut self) -> Option<Statement<'p>> {
        match self.cur_tok()? {
            Token::Let => self.parse_let_stmt(),
            Token::Fn => self.parse_fn_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::Use => todo!(),
            Token::Loop => self.parse_loop_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::LCurly => Some(Statement::Block(self.parse_block_stmt(Token::RCurly))),
            _ => Some(Statement::Expression(self.parse_expr(Precedence::Lowest)?)),
        }
    }

    fn parse_expr(&mut self, prec: Precedence) -> Option<Expression<'p>> {
        let prefix = self.parse_prefix();

        let mut left_expression = prefix;

        while !self.peek_is_end() && prec < self.get_precedence(self.peek_tok()?) {
            self.next_tok();
            left_expression = self.parse_infix(left_expression?);
        }

        left_expression
    }

    fn parse_infix(&mut self, left: Expression<'p>) -> Option<Expression<'p>> {
        match self.cur_tok()? {
            Token::Equals | Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                self.parse_infix_expr(left)
            }
            Token::LParent => Some(Expression::Call(self.parse_call_expr(left)?)),
            // Token::LSquare => self.parse_index_expr(left),
            _ => panic!("Invalid for parsing an infix expr: {:#?}", left),
        }
    }

    fn parse_prefix(&mut self) -> Option<Expression<'p>> {
        match self.cur_tok()? {
            Token::LitInt(int) => Some(Expression::Literal(Literal::Integer(int.parse().unwrap()))),
            Token::LitFloat(float) => {
                Some(Expression::Literal(Literal::Float(float.parse().unwrap())))
            }
            Token::LitString(string) => {
                Some(Expression::Literal(Literal::String(string)))
            }
            //Token::LitBool(boolean) => Some(Expression::Literal(Literal::Boolean(
            //    boolean.parse().unwrap(),
            //))),
            Token::Ident(ident) => Some(Expression::Literal(Literal::Ident(ident))),
            _ => panic!("No prefix parse found for: {:?}", self.cur_tok()),
        }
    }

    fn parse_infix_expr(&mut self, left_expr: Expression<'p>) -> Option<Expression<'p>> {
        let op = self.cur_tok_to_in_op()?;
        let prec = self.get_precedence(self.cur_tok()?);
        self.next_tok();
        let right_expr = self.parse_expr(prec)?;
        Some(Expression::Infix(InfixOpExpr {
            sides: (self.arena.alloc(left_expr), self.arena.alloc(right_expr)),
            operator: op,
        }))
    }

    fn parse_let_stmt(&mut self) -> Option<Statement<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| panic!(
            "Expected peek token to be Ident, received {tok:?} instead"
        ));
        let name = self.parse_typed_ident()?;
        expect_tok!(self.peek_tok(), Some(Token::Assign), |tok| panic!(
            "Expected peek token to be ASSIGN, received {tok:?} instead"
        ));
        // skip name and assign
        self.next_tok();
        self.next_tok();

        let val = self.parse_expr(Precedence::Lowest)?;
        expect_tok!(self.peek_tok(), Some(Token::Semicolon), |tok| panic!(
            "Expected peek token to be SEMICOLON, received {tok:?} instead"
        ));

        // expression value and semicolon
        self.next_tok();
        Some(Statement::Let(LetStatement { name, val }))
    }

    fn parse_fn_stmt(&mut self) -> Option<Statement<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| panic!(
            "Expected peek token to be Ident, received {tok:?} instead"
        ));
        self.next_tok();

        let name = *match self.cur_tok()? {
            Token::Ident(ident) => ident,
            tok => panic!("{tok:?}"),
        };

        expect_tok!(self.peek_tok(), Some(Token::LParent), |tok| panic!(
            "Expected peek token to be LPARENT, received {tok:?} instead"
        ));
        self.next_tok();

        let args = match self.peek_tok()? {
            Token::RParent => {
                self.next_tok();
                Vec::new()
            }
            _ => self.parse_def_args()?,
        };

        expect_tok!(self.peek_tok(), Some(Token::Colon), |tok| panic!(
            "Expected peek token to be COLON, received {tok:?} instead"
        ));

        self.next_tok();

        expect_tok!(
            self.peek_tok(),
            Some(Token::Ident(_))
                | Some(Token::Int)
                | Some(Token::String)
                | Some(Token::Char)
                | Some(Token::Float),
            |tok| panic!("Expected peek token to be IDENT, received {tok:?} instead")
        );
        self.next_tok();

        let ret_type = self.determine_type(self.cur_tok()?);

        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| panic!(
            "Expected peek token to be LCURLY, received {tok:?} instead"
        ));
        self.next_tok();

        let block = self.parse_block_stmt(Token::RCurly);

        // parse_block_stmt(...) sets the current token to the end token
        // so we don't have to do anything here

        Some(Statement::Fn(FnStatement {
            name,
            args,
            ret_type: Type::Ident(ret_type),
            block,
        }))
    }

    fn parse_if_stmt(&mut self) -> Option<Statement<'p>> {
        self.next_tok();
        let condition = self.parse_expr(Precedence::Lowest)?;
        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| panic!(
            "Expected peek token to be LCURLY, received {tok:?} instead"
        ));
        // go to left curly bracket
        self.next_tok();
        let block = self.parse_block_stmt(Token::RCurly);

        self.next_tok();

        Some(Statement::If(IfStatement { condition, block }))
    }

    fn parse_loop_stmt(&mut self) -> Option<Statement<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| panic!(
            "Expected peek token to be LCURLY, received {tok:?} instead"
        ));
        self.next_tok();
        let block = self.parse_block_stmt(Token::RCurly);

        self.next_tok();
        Some(Statement::Loop(LoopStatement {
            condition: None,
            block,
        }))
    }

    fn parse_return_stmt(&mut self) -> Option<Statement<'p>> {
        self.next_tok();
        let val = self.parse_expr(Precedence::Lowest);
        expect_tok!(self.peek_tok(), Some(Token::Semicolon), |tok| panic!(
            "Expected peek token to be SEMICOLON, received {tok:?} instead"
        ));
        self.next_tok();
        dbg!("after ret: {}", self.peek_tok());
        Some(Statement::Return(ReturnStatement { val: val? }))
    }

    /// cur token should be the beginning of the block, for example: `{`
    /// sets cur token to the end token (function argument)
    fn parse_block_stmt(&mut self, end: Token) -> BlockStatement<'p> {
        let mut block = Vec::new();
        self.next_tok();
        while self.cur_tok() != Some(&end) {
            block.push(match self.parse_stmt() {
                Some(stmt) => stmt,
                None => break,
            });
            dbg!(self.peek_tok());
            self.next_tok();
        }

        BlockStatement { stmts: block }
    }

    /// Parses the arguments of a function definition
    ///
    /// cur_token should be beginning of the list, for example `(`
    fn parse_def_args(&mut self) -> Option<Vec<TypedIdent<'p>>> {
        let mut args = Vec::new();
        loop {
            args.push(self.parse_typed_ident()?);
            if let Some(Token::Comma) = self.peek_tok() {
                self.next_tok();
            // FIXME: These else branches are highly sus
            } else if let Some(Token::RParent) = self.cur_tok() {
                break;
            } else if let Some(Token::RParent) = self.peek_tok() {
                break;
            } else {
                expect_tok!(self.peek_tok(), Some(Token::RParent), |tok| panic!(
                    "Expected peek token to be RPARENT, received {tok:?} instead"
                ));
            }
        }
        self.next_tok();

        Some(args)
    }

    /// cur_token should be the token before the first TypedIdent
    ///
    /// cur_token gets set to the type of the ident
    fn parse_typed_ident(&mut self) -> Option<TypedIdent<'p>> {
        // go to ident
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| panic!(
            "Expected peek token to be Ident, received {tok:?} instead"
        ));
        self.next_tok();
        let ident = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            tok => panic!("{tok:?}"),
        };
        // go to colon
        expect_tok!(self.peek_tok(), Some(Token::Colon), |tok| panic!(
            "Expected peek token to be COLON, received {tok:?} instead"
        ));
        self.next_tok();

        self.expect_peek_tok_as_type();
        // go to next ident
        self.next_tok();
        let _type = self.determine_type(self.cur_tok()?);

        Some(TypedIdent { ident, _type: Type::Ident(_type) })
    }

    fn parse_call_expr(&mut self, left: Expression<'p>) -> Option<CallExpression<'p>> {
        let name = match left {
            Expression::Literal(Literal::Ident(var)) => var,
            _ => panic!("{left:?} is not an ident"),
        };

        let args = self.parse_call_args()?;

        Some(CallExpression { name, args })
    }

    // cur token is a Left parenthesis
    fn parse_call_args(&mut self) -> Option<Vec<Expression<'p>>> {
        if let Some(Token::RParent) = self.peek_tok() {
            self.next_tok();
            return None;
        }

        let mut args = Vec::new();

        self.next_tok();

        args.push(self.parse_expr(Precedence::Lowest)?);

        while let Some(Token::Comma) = self.peek_tok() {
            self.next_tok();
            self.next_tok();
            args.push(self.parse_expr(Precedence::Lowest)?);
        }

        self.next_tok();

        Some(args)
    }

    fn expect_peek_tok_as_type(&self) {
        match self.peek_tok() {
            Some(Token::Int) => (),
            Some(Token::Float) => (),
            Some(Token::Char) => (),
            Some(Token::String) => (),
            Some(Token::Ident(_)) => (),
            tok => panic!("expected a type, got: {tok:?}"),
        }
    }

    fn cur_tok_to_in_op(&self) -> Option<Operator> {
        match self.cur_tok()? {
            Token::Plus => Some(Operator::Add),
            Token::Minus => Some(Operator::Sub),
            Token::Divide => Some(Operator::Div),
            Token::Multiply => Some(Operator::Mul),
            Token::Assign => todo!(),
            Token::Equals => Some(Operator::Equals),
            tok => panic!("Cannot convert {tok:?} to operator"),
        }
    }

    fn peek_is_end(&self) -> bool {
        matches!(self.peek_tok(), Some(Token::Semicolon) | None)
    }

    fn get_precedence(&self, token: &Token<'p>) -> Precedence {
        match token {
            Token::Assign => Precedence::Assign,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Multiply | Token::Divide => Precedence::Prod,
            Token::Equals => Precedence::Equals,
            Token::LParent => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn determine_type(&self, tok: &'p Token<'p>) -> &'p str {
        match tok {
            Token::Ident(ident) => ident,
            Token::Int => "int",
            Token::Float => "float",
            Token::String => "string",
            Token::Char => "char",
            tok => panic!("Invalid token for type: {tok:?}"),
        }
    }

    #[inline(always)]
    fn cur_tok(&self) -> Option<&'p Token<'p>> {
        self.lexer.tokens.get(self.tok_index)
    }

    #[inline(always)]
    fn peek_tok(&self) -> Option<&'p Token<'p>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    #[inline(always)]
    fn next_tok(&mut self) {
        self.tok_index += 1;
    }
}
