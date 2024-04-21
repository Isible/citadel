//! Parser for parsing list of tokens into list of actually related AST nodes

use std::{collections::HashMap, vec};

use citadel_frontend::ir::{
    self, ArithOpExpr, BlockStmt, BreakStmt, CallExpr, DeclFuncStmt, ExitStmt, FuncStmt, IRExpr,
    IRStmt, IRTypedIdent, Ident, JumpStmt, LabelStmt, Operator, ReturnStmt, VarStmt,
};

use crate::{expect_tok, lexer::Lexer, parser_error, tokens::Token};

pub struct Parser<'l> {
    lexer: &'l Lexer<'l>,

    tok_index: usize,
    pub symbols: HashMap<String, IRStmt>,
}

impl<'l> Parser<'l> {
    pub fn new(lexer: &'l Lexer) -> Self {
        Self {
            lexer,
            tok_index: 0,
            symbols: HashMap::new(),
        }
    }

    pub fn parse_program(&mut self) -> Vec<IRStmt> {
        let mut program = Vec::new();
        while let Some(stmt) = self.parse_stmt() {
            program.push(stmt);
            self.next_tok();
        }
        program
    }

    pub fn parse_stmt(&mut self) -> Option<IRStmt> {
        match self.cur_tok()? {
            Token::DollarSign => self.parse_variable(true),
            Token::QuestionMark => self.parse_variable(false),
            Token::At => self.parse_function(),
            Token::Apostrophe => self.parse_label(),
            Token::Decl => self.parse_function_decl(),
            Token::Call => self.parse_call().map(|call| IRStmt::Call(call)),
            Token::Ret => self.parse_return(),
            Token::Exit => self.parse_exit(),
            Token::Break => self.parse_break(),
            Token::Jump => self.parse_jump(),
            tok => panic!("Cannot parse statement from token: {tok:?}"),
        }
    }

    pub fn parse_expr(&mut self) -> Option<IRExpr> {
        match self.cur_tok()? {
            Token::Call => self.parse_call().map(|call| IRExpr::Call(call)),
            Token::Add => self.parse_arith_op_expr(Operator::Add),
            Token::Sub => self.parse_arith_op_expr(Operator::Sub),
            Token::Mul => self.parse_arith_op_expr(Operator::Mul),
            Token::Div => self.parse_arith_op_expr(Operator::Div),
            Token::LitString(str) => Some(IRExpr::Literal(ir::Literal::String((*str).into()))),
            Token::LitInt(int) => {
                dbg!(int);
                Some(IRExpr::Literal(ir::Literal::Int32(
                    int.parse::<i32>().unwrap(),
                )))
            }
            Token::LitChar(ch) => Some(IRExpr::Literal(ir::Literal::Char(
                ch.chars().nth(0).unwrap(),
            ))),
            Token::PercentSign => self.parse_ident(),
            Token::Ident(ident) => Some(IRExpr::Ident(Ident(ident.to_string()))),
            tok => todo!("cur tok: {tok:?}"),
        }
    }

    fn parse_ident(&mut self) -> Option<IRExpr> {
        self.next_tok();
        self.parse_expr()
    }

    fn parse_variable(&mut self, is_const: bool) -> Option<IRStmt> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));

        self.next_tok();

        let ident = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the type, received {tok:?} instead"
        ));

        self.next_tok();

        let _type = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok(), Some(Token::Assign), |tok| parser_error!(
            "Expected peek token to be Assign, received {tok:?} instead"
        ));

        self.next_tok();

        self.next_tok();

        let val = self.parse_expr();
        let var = IRStmt::Variable(VarStmt {
            name: IRTypedIdent {
                ident: ident.into(),
                _type: _type.into(),
            },
            val: val?,
            is_const,
        });
        self.symbols.insert(ident.into(), var.clone());
        Some(var)
    }

    fn parse_function(&mut self) -> Option<IRStmt> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));
        self.next_tok();

        let name = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok(), Some(Token::LParent), |tok| {
            parser_error!(
            "Expected peek token to be a left parenthesis for declaring function arguments, received {tok:?} instead"
        )
        });

        self.next_tok();

        let args = self.parse_arg_list(Token::RParent);

        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the type, received {tok:?} instead"
        ));

        self.next_tok();

        let _type = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| {
            parser_error!(
            "Expected peek token to be a left curly bracket specifying the function block, received {tok:?} instead"
        )
        });

        self.next_tok();

        let block = self.parse_block();

        let func = IRStmt::Function(FuncStmt {
            name: IRTypedIdent {
                ident: name.into(),
                _type: _type.into(),
            },
            args: args?,
            block: block?,
        });
        self.symbols.insert(name.into(), func.clone());
        Some(func)
    }

    fn parse_function_decl(&mut self) -> Option<IRStmt> {
        expect_tok!(self.peek_tok(), Some(Token::At), |tok| {
            parser_error!(
            "Expected peek token to be an At specifying that this is a function, received {tok:?} instead"
        )
        });
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));
        let name = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };
        expect_tok!(self.peek_tok(), Some(Token::LParent), |tok| {
            parser_error!(
            "Expected peek token to be a left parenthesis declaring the arguments, received {tok:?} instead"
        )
        });
        self.next_tok();
        let args = self.parse_arg_list(Token::RParent);
        self.next_tok();
        // TODO: dbg!(self.cur_tok());
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the type, received {tok:?} instead"
        ));
        self.next_tok();
        let _type = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };
        Some(IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent {
                ident: name.into(),
                _type: _type.into(),
            },
            args: args?,
        }))
    }

    fn parse_label(&mut self) -> Option<IRStmt> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| {
            parser_error!(
            "Expected peek token to be an identifier specifying the label name, received {tok:?} instead"
        )
        });
        self.next_tok();
        let name = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };
        expect_tok!(self.peek_tok(), Some(Token::Colon), |tok| parser_error!(
            "Expected peek token to be a colon, received {tok:?} instead"
        ));
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| {
            parser_error!(
            "Expected peek token to be a left curly bracket, declaring the label block, received {tok:?} instead"
        )
        });
        self.next_tok();
        let block = self.parse_block();
        let label = IRStmt::Label(LabelStmt {
            name: name.into(),
            block: block?,
        });
        self.symbols.insert(name.into(), label.clone());
        Some(label)
    }

    fn parse_call(&mut self) -> Option<CallExpr> {
        expect_tok!(
            self.peek_tok(),
            Some(Token::PercentSign),
            |tok| parser_error!(
                "Expected peek token to be a percent sign, received {tok:?} instead"
            )
        );
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| {
            parser_error!(
            "Expected peek token to be an identifier specifying the function name, received {tok:?} instead"
        )
        });
        self.next_tok();
        let name = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };
        expect_tok!(self.peek_tok(), Some(Token::LParent), |tok| {
            parser_error!(
            "Expected peek token to be a left parenthesis for declaring the call arguments, received {tok:?} instead"
        )
        });
        self.next_tok();
        let args = self.parse_expr_list(Token::RParent);
        Some(CallExpr {
            name: name.into(),
            args: args?,
        })
    }

    fn parse_return(&mut self) -> Option<IRStmt> {
        self.next_tok();
        let expr = self.parse_expr();
        Some(IRStmt::Return(ReturnStmt { ret_val: expr? }))
    }

    fn parse_exit(&mut self) -> Option<IRStmt> {
        self.next_tok();
        let code = self.parse_expr()?;
        Some(IRStmt::Exit(ExitStmt { exit_code: code }))
    }

    fn parse_break(&mut self) -> Option<IRStmt> {
        expect_tok!(
            self.peek_tok(),
            Some(Token::Apostrophe),
            |tok| parser_error!(
                "Expected peek token to be an apostrophe, received {tok:?} instead"
            )
        );
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an ident specifying the label name, received {tok:?} instead"
        ));
        self.next_tok();
        let label = self.parse_label_ref();
        Some(IRStmt::Break(BreakStmt {
            label: label?.to_string(),
        }))
    }

    fn parse_jump(&mut self) -> Option<IRStmt> {
        expect_tok!(
            self.peek_tok(),
            Some(Token::Apostrophe),
            |tok| parser_error!(
                "Expected peek token to be an apostrophe, received {tok:?} instead"
            )
        );
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an ident specifying the label name, received {tok:?} instead"
        ));
        self.next_tok();
        let label = self.parse_label_ref();
        Some(IRStmt::Jump(JumpStmt {
            label: label?.to_string(),
        }))
    }

    fn parse_arith_op_expr(&mut self, op: Operator) -> Option<IRExpr> {
        self.next_tok();
        let left = self.parse_expr();

        expect_tok!(self.peek_tok(), Some(Token::Comma), |tok| parser_error!(
            "Expected peek token to be a comma, received {tok:?} instead"
        ));

        self.next_tok();
        self.next_tok();
        let right = self.parse_expr();

        Some(IRExpr::ArithOp(ArithOpExpr {
            op,
            values: (Box::from(left?), Box::from(right?)),
        }))
    }

    fn parse_label_ref(&mut self) -> Option<&str> {
        let name = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };
        self.next_tok();
        Some(name)
    }

    fn parse_block(&mut self) -> Option<BlockStmt> {
        let mut block = Vec::new();
        while self.peek_tok() != Some(&Token::RCurly) {
            self.next_tok();
            block.push(match self.parse_stmt() {
                Some(stmt) => stmt,
                None => return None,
            });
        }
        self.next_tok();
        Some(BlockStmt { stmts: block })
    }

    fn parse_expr_list(&mut self, end: Token<'l>) -> Option<Vec<IRExpr>> {
        if self.peek_tok() == Some(&end) {
            self.next_tok();
            return Some(vec![]);
        }
        let mut args = Vec::new();
        self.next_tok();
        loop {
            args.push(match self.parse_expr() {
                Some(expr) => expr,
                None => return None,
            });
            if let Some(Token::Comma) = self.peek_tok() {
                self.next_tok();
                self.next_tok();
            } else if self.peek_tok() == Some(&end) {
                break;
            } else {
                expect_tok!(self.peek_tok(), Some(Token::RParent), |tok| parser_error!(
                    "Expected peek token to be a right parent, received {tok:?} instead"
                ));
            }
        }
        self.next_tok();

        Some(args)
    }

    fn parse_arg_list(&mut self, end: Token) -> Option<Vec<IRTypedIdent>> {
        if self.peek_tok() == Some(&end) {
            self.next_tok();
            return Some(vec![]);
        }
        let mut args = Vec::new();
        self.next_tok();
        loop {
            expect_tok!(self.cur_tok(), Some(Token::DollarSign), |tok| panic!(
                "Expected dollar sign, received {tok:?} instead"
            ));
            self.next_tok();
            args.push(match self.parse_typed_ident() {
                Some(ident) => ident,
                None => return None,
            });
            if let Some(Token::Comma) = self.peek_tok() {
                self.next_tok();
                self.next_tok();
            } else if self.peek_tok() == Some(&end) {
                break;
            } else {
                expect_tok!(self.peek_tok(), Some(Token::RParent), |tok| parser_error!(
                    "Expected peek token to be a right parent, received {tok:?} instead"
                ));
            }
        }
        self.next_tok();

        Some(args)
    }

    fn parse_typed_ident(&mut self) -> Option<IRTypedIdent> {
        let ident = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => panic!("Expected ident for the name"),
        };
        self.next_tok();
        let _type = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => panic!("Expected ident for the type"),
        };
        Some(IRTypedIdent {
            ident: ident.to_string(),
            _type: _type.to_string(),
        })
    }

    #[inline(always)]
    fn cur_tok(&self) -> Option<&Token<'l>> {
        self.lexer.tokens.get(self.tok_index)
    }

    #[inline(always)]
    fn peek_tok(&self) -> Option<&Token<'l>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    #[inline(always)]
    fn next_tok(&mut self) {
        self.tok_index += 1;
    }
}
