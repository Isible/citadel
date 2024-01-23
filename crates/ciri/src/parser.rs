use std::mem::swap;

use frontend::ast::{IRStmt, ConstStmt, IRTypedIdent, IRExpr, VarStmt};

use crate::{lexer::Lexer, tokens::{Token, Literal}};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = lexer.tokenize();
        lexer.next_char();
        let peek_tok = lexer.tokenize();

        Self {
            lexer,
            cur_tok,
            peek_tok,
        }
    }

    pub fn parse_stmt(&mut self) -> IRStmt {
        match self.cur_tok {
            Token::DollarSign => self.parse_variable(),
            Token::QuestionMark => self.parse_variable(),
            Token::At => self.parse_function(),
            Token::PercentSign => todo!(),
            Token::Assign => todo!(),
            Token::Apostrophe => todo!(),
            Token::Colon => todo!(),
            Token::Dot => todo!(),
            Token::LParent => todo!(),
            Token::RParent => todo!(),
            Token::LSquare => todo!(),
            Token::RSquare => todo!(),
            Token::LCurly => todo!(),
            Token::RCurly => todo!(),
            Token::Lcl => todo!(),
            Token::Pub => todo!(),
            Token::Abst => todo!(),
            Token::Call => todo!(),
            Token::Ret => todo!(),
            Token::Add => todo!(),
            Token::Sub => todo!(),
            Token::Mul => todo!(),
            Token::Div => todo!(),
            Token::RawLit(_) => todo!(),
            Token::Lit(_) => todo!(),
            Token::Ident(_) => todo!(),
            Token::Eof => todo!(),
        }
    }

    pub fn parse_expr(&mut self) -> IRExpr {
        match self.cur_tok {
            Token::Call => todo!(),
            Token::Add => todo!(),
            Token::Sub => todo!(),
            Token::Mul => todo!(),
            Token::Div => todo!(),
            Token::Lit(_) => self.parse_lit(),
            Token::Ident(_) => todo!(),
            _ => todo!(),
        }
    }

    fn parse_lit(&mut self) -> IRExpr {
        let lit = match self.cur_tok {
            Token::Lit(ref lit) => lit,
            _ => panic!(),
        };
        IRExpr::Literal(match lit {
            Literal::String(_) => frontend::ast::Literal::String(self.cur_tok.to_string()),
            Literal::Integer(int) => frontend::ast::Literal::Integer(32, *int as isize),
            Literal::Float(float) => frontend::ast::Literal::LongFloat(64, *float),
            Literal::Boolean(bool) => frontend::ast::Literal::Bool(*bool),
            Literal::Char(ch) => frontend::ast::Literal::Char(*ch),
            Literal::Array(_) => todo!(),
            Literal::Vector(_) => todo!(),
        })
    }

    fn parse_variable(&mut self) -> IRStmt {
        let is_const = match self.cur_tok {
            Token::DollarSign => true,
            Token::QuestionMark => false,
            _ => panic!()
        };

        dbg!("cur {}, next {}", &self.cur_tok, &self.peek_tok);

        match self.peek_tok {
            Token::Ident(_) => (),
            _ => panic!("Expect peek token to be an Identifier, received {} instead", self.peek_tok),
        }

        self.next_token();

        let name = self.cur_tok.to_string();

        if self.peek_tok != Token::Lcl && self.peek_tok != Token::Pub {
            panic!("Expected access modifier like pub or lcl, received {:?} instead", self.peek_tok);
        }

        self.next_token();

        let is_local = match self.cur_tok {
            Token::Lcl => true,
            Token::Pub => false,
            // unreachable due to previous token check
            _ => panic!(),
        };

        // expect next token
        match self.peek_tok {
            Token::Ident(_) => (),
            _ => panic!("Expect peek token to be an Identifier, received {} instead", self.peek_tok),
        }

        self.next_token();

        let _type = self.cur_tok.to_string();

        self.expect_peek_tok(Token::Assign);

        self.next_token();

        self.next_token();

        let val = self.parse_expr();

        match is_const {
            true => IRStmt::Constant(ConstStmt {
                name: IRTypedIdent {
                    ident: name,
                    _type,
                },
                val,
                is_local,
            }),
            false => IRStmt::Variable(VarStmt {
                name: IRTypedIdent {
                    ident: name,
                    _type,
                },
                val,
                is_local,
            }),
        }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.lexer.next_char();
        self.peek_tok = self.lexer.tokenize();
    }

    fn expect_peek_tok(&self, expect: Token) {
        if self.peek_tok != expect {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }
}
