//! Parser for parsing list of tokens into list of actually related AST nodes

use std::{collections::HashMap, mem::swap};

use citadel_frontend::ir::{
    self, ArithOpExpr, BlockStmt, BreakStmt, CallExpr, DeclFuncStmt, FuncStmt, IRExpr, IRStmt,
    IRTypedIdent, JumpStmt, LabelStmt, Operator, ReturnStmt, VarStmt,
};

use crate::{
    lexer::Lexer,
    tokens::{Literal, Token},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
    pub symbols: HashMap<String, IRStmt>,
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
            symbols: HashMap::new(),
        }
    }

    pub fn parse_program(&mut self) -> Vec<IRStmt> {
        let mut program = Vec::new();
        while let Some(stmt) = self.parse_stmt() {
            program.push(stmt);
            self.next_token();
        }
        program
    }

    pub fn parse_stmt(&mut self) -> Option<IRStmt> {
        Some(match self.cur_tok {
            Token::DollarSign => self.parse_variable(true),
            Token::QuestionMark => self.parse_variable(false),
            Token::At => self.parse_function(),
            Token::Apostrophe => self.parse_label(),
            Token::Decl => self.parse_function_decl(),
            Token::Call => IRStmt::Call(self.parse_call()),
            Token::Ret => self.parse_return(),
            Token::Break => self.parse_break(),
            Token::Jump => self.parse_jump(),
            Token::Eof => return None,
            _ => panic!("Cannot parse statement from token: {}", self.cur_tok),
        })
    }

    pub fn parse_expr(&mut self) -> IRExpr {
        match self.cur_tok {
            Token::Call => IRExpr::Call(self.parse_call()),
            Token::Add => self.parse_arith_op_expr(Operator::Add),
            Token::Sub => self.parse_arith_op_expr(Operator::Sub),
            Token::Mul => self.parse_arith_op_expr(Operator::Mul),
            Token::Div => self.parse_arith_op_expr(Operator::Div),
            Token::Lit(_) => self.parse_lit(),
            Token::Ident(ref ident) => IRExpr::Ident(ident.to_string()),
            _ => todo!("cur tok: {:?}", self.cur_tok),
        }
    }

    fn parse_lit(&mut self) -> IRExpr {
        let lit = match self.cur_tok {
            Token::Lit(ref lit) => lit,
            _ => panic!(),
        };
        IRExpr::Literal(match lit {
            Literal::String(ref str) => ir::Literal::String(str.into()),
            Literal::Integer(int) => ir::Literal::Int64(*int),
            Literal::Float(float) => ir::Literal::Double(*float),
            Literal::Boolean(bool) => ir::Literal::Bool(*bool),
            Literal::Char(ch) => ir::Literal::Char(*ch),
            Literal::Array(_) => todo!(),
            Literal::Vector(_) => todo!(),
        })
    }

    fn parse_variable(&mut self, is_const: bool) -> IRStmt {
        match self.peek_tok {
            Token::Ident(_) => (),
            _ => panic!(
                "Expect peek token to be an Identifier, received {} instead",
                self.peek_tok
            ),
        }

        self.next_token();

        let name = self.cur_tok.to_string();

        if self.peek_tok != Token::Priv && self.peek_tok != Token::Pub {
            panic!(
                "Expected access modifier like pub or lcl, received {:?} instead",
                self.peek_tok
            );
        }

        self.next_token();

        let is_local = match self.cur_tok {
            Token::Priv => true,
            Token::Pub => false,
            // unreachable due to previous token check
            _ => panic!(),
        };

        // expect next token
        match self.peek_tok {
            Token::Ident(_) => (),
            _ => panic!(
                "Expect peek token to be an Identifier, received {} instead",
                self.peek_tok
            ),
        }

        self.next_token();

        let _type = self.cur_tok.to_string();

        self.expect_peek_tok(&Token::Assign);

        self.next_token();

        self.next_token();

        let val = self.parse_expr();
        let var = IRStmt::Variable(VarStmt {
            name: IRTypedIdent {
                ident: name.clone(),
                _type,
            },
            val,
            is_local,
            is_const,
        });
        self.symbols.insert(name, var.clone());
        var
    }

    fn parse_function(&mut self) -> IRStmt {
        self.next_token();

        let name = match self.cur_tok {
            Token::Ident(_) => self.cur_tok.to_string(),
            _ => panic!("Expected identifier after @ for the function name"),
        };

        self.next_token();

        let args = self.parse_arg_list(Token::RParent);

        let is_local = match self.peek_tok {
            Token::Priv => true,
            Token::Pub => false,
            _ => panic!("Expected pub or priv, got {} instead", self.peek_tok),
        };

        self.next_token();

        self.next_token();

        let _type = self.cur_tok.to_string();

        self.expect_peek_tok(&Token::LCurly);

        self.next_token();

        let block = self.parse_block();

        let func = IRStmt::Function(FuncStmt {
            name: IRTypedIdent {
                ident: name.clone(),
                _type,
            },
            args,
            block,
            is_local,
        });
        self.symbols.insert(name, func.clone());
        func
    }

    fn parse_function_decl(&mut self) -> IRStmt {
        self.expect_peek_tok(&Token::At);
        self.next_token();
        let name = match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!("Expected ident after decl, got {} instead", self.peek_tok),
        };
        self.next_token();
        self.expect_peek_tok(&Token::LParent);
        self.next_token();
        let args = self.parse_arg_list(Token::RParent);
        let is_local = match self.peek_tok {
            Token::Priv => true,
            Token::Pub => false,
            _ => panic!("Expected pub or lcl, got {} instead", self.peek_tok),
        };
        self.next_token();
        let _type = match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!("Expected ident for a type, got {} instead", self.peek_tok),
        };
        IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent { ident: name, _type },
            args,
            is_local,
        })
    }

    fn parse_label(&mut self) -> IRStmt {
        self.next_token();
        let name = match self.cur_tok {
            Token::Ident(_) => self.cur_tok.to_string(),
            _ => panic!(
                "Expected ident after label declaration, got {} instead",
                self.cur_tok
            ),
        };
        self.expect_peek_tok(&Token::Colon);
        self.next_token();
        self.expect_peek_tok(&Token::LCurly);
        self.next_token();
        let block = self.parse_block();
        let label = IRStmt::Label(LabelStmt {
            name: name.clone(),
            block,
        });
        self.symbols.insert(name, label.clone());
        label
    }

    fn parse_call(&mut self) -> CallExpr {
        self.expect_peek_tok(&Token::PercentSign);
        self.next_token();
        let name = match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!(
                "Expected ident for call function name, got {} instead",
                self.peek_tok
            ),
        };
        self.next_token();
        self.expect_peek_tok(&Token::LParent);
        self.next_token();
        let args = self.parse_expr_list(Token::RParent);
        CallExpr { name, args }
    }

    fn parse_return(&mut self) -> IRStmt {
        self.next_token();
        let expr = self.parse_expr();
        IRStmt::Return(ReturnStmt { ret_val: expr })
    }

    fn parse_break(&mut self) -> IRStmt {
        self.expect_peek_tok(&Token::Apostrophe);
        self.next_token();
        let label = self.parse_label_ref();
        IRStmt::Break(BreakStmt { label })
    }

    fn parse_jump(&mut self) -> IRStmt {
        self.expect_peek_tok(&Token::Apostrophe);
        self.next_token();
        let label = self.parse_label_ref();
        IRStmt::Jump(JumpStmt { label })
    }

    fn parse_arith_op_expr(&mut self, op: Operator) -> IRExpr {
        self.next_token();
        let left = self.parse_expr();
        self.expect_peek_tok(&Token::Comma);
        self.next_token();
        self.next_token();
        let right = self.parse_expr();
        IRExpr::ArithOp(ArithOpExpr {
            op,
            values: (Box::from(left), Box::from(right)),
        })
    }

    fn parse_label_ref(&mut self) -> String {
        let name = match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!("Expected label name after apostrophe"),
        };
        self.next_token();
        name
    }

    fn parse_block(&mut self) -> BlockStmt {
        let mut block = Vec::new();
        while self.peek_tok != Token::RCurly {
            self.next_token();
            block.push(
                self.parse_stmt()
                    .unwrap_or_else(|| panic!("Encountered EOF before end of block statement")),
            );
        }
        self.next_token();
        BlockStmt { stmts: block }
    }

    fn parse_expr_list(&mut self, end: Token) -> Vec<IRExpr> {
        if self.peek_tok == end {
            self.next_token();
            return Vec::new();
        }
        let mut args = Vec::new();
        self.next_token();
        loop {
            args.push(self.parse_expr());
            if self.peek_tok == Token::Comma {
                self.next_token();
                self.next_token();
            } else if self.peek_tok == end {
                break;
            } else {
                self.expect_peek_tok(&Token::RParent);
            }
        }
        self.next_token();

        args
    }

    fn parse_arg_list(&mut self, end: Token) -> Vec<IRTypedIdent> {
        if self.peek_tok == end {
            self.next_token();
            return Vec::new();
        }
        let mut args = Vec::new();
        self.next_token();
        loop {
            args.push(self.parse_typed_ident());
            if self.peek_tok == Token::Comma {
                self.next_token();
                self.next_token();
            } else if self.peek_tok == end {
                break;
            } else {
                self.expect_peek_tok(&end);
            }
        }
        self.next_token();

        args
    }

    fn parse_typed_ident(&mut self) -> IRTypedIdent {
        let ident = self.cur_tok.to_string();
        self.next_token();
        let _type = self.cur_tok.to_string();
        IRTypedIdent { ident, _type }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.lexer.next_char();
        self.peek_tok = self.lexer.tokenize();
    }

    fn expect_peek_tok(&self, expect: &Token) {
        if &self.peek_tok != expect {
            panic!("expected: {:?}, received: {:?}", expect, self.peek_tok)
        }
    }
}
