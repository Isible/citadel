//! Parser for parsing list of tokens into list of actually related AST nodes

use std::collections::HashMap;

use bumpalo::Bump;
use citadel_frontend::hir::{
    self,
    irgen::{HIRStream, IRGenerator},
    ArithOpExpr, BlockStmt, CallExpr, DeclFuncStmt, ExitStmt, FuncStmt, IRExpr, IRStmt,
    IRTypedIdent, Ident, JumpStmt, LabelStmt, Literal, Operator, ReturnStmt, StructInitExpr,
    StructStmt, UnionStmt, VarStmt,
};

use crate::{expect_tok, lexer::Lexer, parser_error, tokens::Token};

pub struct Parser<'p> {
    lexer: &'p Lexer<'p>,
    arena: &'p Bump,

    tok_index: usize,
    pub symbols: HashMap<&'p str, IRStmt<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: &'p Lexer, arena: &'p Bump) -> Self {
        Self {
            lexer,
            arena,
            tok_index: 0,
            symbols: HashMap::new(),
        }
    }

    pub fn parse_program(&mut self) -> HIRStream<'p> {
        let mut ir_gen = IRGenerator::default();
        while let Some(stmt) = self.parse_stmt() {
            ir_gen.gen_ir(stmt);
            self.next_tok();
        }
        ir_gen.stream()
    }

    pub fn parse_stmt(&mut self) -> Option<IRStmt<'p>> {
        match self.cur_tok()? {
            Token::Entry => self.parse_entry(),
            Token::DollarSign => self.parse_variable(true),
            Token::QuestionMark => self.parse_variable(false),
            Token::Func => self.parse_function(),
            Token::Apostrophe => self.parse_label(),
            Token::Decl => self.parse_function_decl(),
            Token::Call => self.parse_call().map(|call| IRStmt::Call(call)),
            Token::Ret => self.parse_return(),
            Token::Exit => self.parse_exit(),
            Token::Jump => self.parse_jump(),
            Token::Struct => self.parse_struct(),
            Token::Union => self.parse_union(),
            tok => panic!(
                "Cannot parse statement from token: {tok:?} (peek: {:?})",
                self.peek_tok()
            ),
        }
    }

    pub fn parse_expr(&mut self) -> Option<IRExpr<'p>> {
        match self.cur_tok()? {
            Token::Call => self.parse_call().map(|call| IRExpr::Call(call)),
            Token::Add => self.parse_arith_op_expr(Operator::Add),
            Token::Sub => self.parse_arith_op_expr(Operator::Sub),
            Token::Mul => self.parse_arith_op_expr(Operator::Mul),
            Token::Div => self.parse_arith_op_expr(Operator::Div),
            Token::Ident("l") if *self.peek_tok()? == Token::LCurly => self.parse_lit(),
            Token::PercentSign => self.parse_ident(),
            Token::Struct => self.parse_struct_init(),
            tok => todo!("cur tok: {tok:?}"),
        }
    }

    fn parse_entry(&mut self) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok()?, Token::LCurly, |tok| {
            parser_error!(
                "Expected left curly starting block after entry keyword, received {tok:?} instead"
            );
        });
        self.next_tok();
        let block = self.parse_block()?;
        Some(IRStmt::Entry(block))
    }

    fn parse_ident(&mut self) -> Option<IRExpr<'p>> {
        self.next_tok();
        match self.cur_tok()? {
            Token::Ident(ident) => Some(IRExpr::Ident(ident)),
            _ => panic!(),
        }
    }

    fn parse_variable(&mut self, is_const: bool) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));

        self.next_tok();

        let ident = match self.cur_tok() {
            Some(Token::Ident(ident)) => *ident,
            _ => unreachable!(),
        };

        self.next_tok();

        let _type = self.parse_type()?;

        expect_tok!(self.peek_tok(), Some(Token::Assign), |tok| parser_error!(
            "Expected peek token to be Assign, received {tok:?} instead"
        ));

        self.next_tok();

        self.next_tok();

        let val = self.parse_expr()?;
        let var = IRStmt::Variable(VarStmt {
            name: IRTypedIdent {
                ident: ident,
                _type,
            },
            val,
            is_const,
        });
        self.symbols.insert(ident, var.clone());
        Some(var)
    }

    fn parse_struct(&mut self) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok()?, Token::At, |tok| parser_error!(
            "Expected peek token to be an @, received {tok:?} instead"
        ));
        self.next_tok();
        expect_tok!(self.peek_tok()?, Token::Ident(_), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));
        self.next_tok();

        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok()?, Token::LCurly, |tok| {
            parser_error!(
            "Expected peek token to be a lcurly declaring the block containing the struct fields, received {tok:?} instead"
        )
        });
        self.next_tok();

        let fields = self.parse_arg_list(Token::RCurly)?;
        dbg!(self.cur_tok());

        Some(IRStmt::Struct(StructStmt { name, fields }))
    }

    fn parse_struct_init(&mut self) -> Option<IRExpr<'p>> {
        expect_tok!(
            self.peek_tok(),
            Some(Token::PercentSign),
            |tok| parser_error!("Expected peek token to be an %, received {tok:?} instead")
        );
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| {
            parser_error!(
            "Expected peek token to be an ident specifying the name of the struct that is being initialized, received {tok:?} instead"
        )
        });
        self.next_tok();

        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| {
            parser_error!("Expected peek token to be a left curly brace, received {tok:?} instead")
        });
        self.next_tok();

        let values = self.parse_expr_list(Token::RCurly)?;

        Some(IRExpr::StructInit(StructInitExpr { name, values }))
    }

    fn parse_union(&mut self) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok()?, Token::At, |tok| parser_error!(
            "Expected peek token to be an @, received {tok:?} instead"
        ));
        self.next_tok();
        expect_tok!(self.peek_tok()?, Token::Ident(_), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));
        self.next_tok();

        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };

        expect_tok!(self.peek_tok()?, Token::LCurly, |tok| {
            parser_error!(
            "Expected peek token to be a lcurly declaring the block containing the struct fields, received {tok:?} instead"
        )
        });
        self.next_tok();

        let variants = self.parse_arg_list(Token::RCurly)?;

        Some(IRStmt::Union(UnionStmt { name, variants }))
    }

    fn parse_function(&mut self) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::At), |tok| parser_error!(
            "Expected peek token to be an @, received {tok:?} instead"
        ));
        self.next_tok();
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

        let _type = self.parse_type()?;

        expect_tok!(self.peek_tok(), Some(Token::LCurly), |tok| {
            parser_error!(
            "Expected peek token to be a left curly bracket specifying the function block, received {tok:?} instead"
        )
        });

        self.next_tok();

        let block = self.parse_block();

        let func = IRStmt::Function(FuncStmt {
            name: IRTypedIdent {
                ident: name,
                _type,
            },
            args: args?,
            block: block?,
        });
        self.symbols.insert(name, func.clone());
        Some(func)
    }

    fn parse_function_decl(&mut self) -> Option<IRStmt<'p>> {
        expect_tok!(self.peek_tok(), Some(Token::At), |tok| {
            parser_error!(
            "Expected peek token to be an At specifying that this is a function, received {tok:?} instead"
        )
        });
        self.next_tok();
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the name, received {tok:?} instead"
        ));
        let ident = match self.cur_tok() {
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
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Expected peek token to be an identifier specifying the type, received {tok:?} instead"
        ));
        self.next_tok();
        let _type = self.parse_type()?;
        Some(IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent {
                ident,
                _type,
            },
            args: args?,
        }))
    }

    fn parse_label(&mut self) -> Option<IRStmt<'p>> {
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
        let label = LabelStmt { name: name };
        self.symbols.insert(name, IRStmt::Label(label));
        Some(IRStmt::Label(label))
    }

    fn parse_call(&mut self) -> Option<CallExpr<'p>> {
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
        let name = self.parse_identifier();
        expect_tok!(self.peek_tok(), Some(Token::LParent), |tok| {
            parser_error!(
            "Expected peek token to be a left parenthesis for declaring the call arguments, received {tok:?} instead"
        )
        });
        self.next_tok();
        let args = self.parse_expr_list(Token::RParent);
        Some(CallExpr {
            name: name?,
            args: args?,
        })
    }

    fn parse_identifier(&self) -> Option<Ident<'p>> {
        match self.cur_tok()? {
            Token::Ident(ident) => Some(*ident),
            _ => unreachable!(),
        }
    }

    fn parse_type(&mut self) -> Option<hir::Type<'p>> {
        match self.cur_tok()? {
            Token::LSquare => self.parse_arr_type(),
            Token::Ident(ident) => Some(hir::Type::Ident(ident)),
            tok => parser_error!("Failed to parse type from token: {tok:?}"),
        }
    }

    fn parse_arr_type(&mut self) -> Option<hir::Type<'p>> {
        self.next_tok();
        let type_ = match *self.cur_tok()? {
            Token::LSquare => self.parse_arr_type()?,
            Token::Ident(ident) => hir::Type::Ident(ident),
            tok => parser_error!("Failed to parse type for array from token: {tok:?}"),
        };
        expect_tok!(self.peek_tok()?, Token::Semicolon, |tok| {
            parser_error!("Expected semicolon after type for array, received: {tok:?} instead")
        });
        self.next_tok();
        let size = match *self.peek_tok()? {
            Token::LitInt(int) => int.parse::<u32>().unwrap(),
            tok => {
                parser_error!("Expected integer literal for array size, received {tok:?} instead")
            }
        };
        self.next_tok();
        expect_tok!(self.peek_tok()?, Token::RSquare, |tok| {
            parser_error!(
                "Expected right square bracket after array size, received {tok:?} instead"
            );
        });
        self.next_tok();
        let type_ref = self.arena.alloc(type_);
        Some(hir::Type::Array(type_ref, size))
    }

    fn parse_return(&mut self) -> Option<IRStmt<'p>> {
        self.next_tok();
        let expr = self.parse_expr();
        Some(IRStmt::Return(ReturnStmt { ret_val: expr? }))
    }

    fn parse_exit(&mut self) -> Option<IRStmt<'p>> {
        self.next_tok();
        let code = self.parse_expr()?;
        Some(IRStmt::Exit(ExitStmt { exit_code: code }))
    }

    fn parse_jump(&mut self) -> Option<IRStmt<'p>> {
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
        let label = match self.cur_tok()? {
            Token::Ident(id) => id,
            _ => unreachable!()
        };
        Some(IRStmt::Jump(JumpStmt { label }))
    }

    fn parse_arith_op_expr(&mut self, op: Operator) -> Option<IRExpr<'p>> {
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

    fn parse_label_ref(&mut self) -> Option<Ident<'p>> {
        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };
        self.next_tok();
        Some(name)
    }

    /// First token is left curly
    fn parse_block(&mut self) -> Option<BlockStmt<'p>> {
        let mut block = Vec::new();
        while self.peek_tok() != Some(&Token::RCurly) {
            self.next_tok();
            block.push(self.parse_stmt()?);
        }
        self.next_tok();
        Some(BlockStmt { stmts: block })
    }

    fn parse_expr_list(&mut self, end: Token<'p>) -> Option<Vec<IRExpr<'p>>> {
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

    fn parse_arg_list(&mut self, end: Token) -> Option<Vec<IRTypedIdent<'p>>> {
        if self.peek_tok() == Some(&end) {
            self.next_tok();
            return Some(vec![]);
        }
        let mut args = Vec::new();
        self.next_tok();
        loop {
            if self.cur_tok() == Some(&end) {
                return Some(args);
            }

            expect_tok!(
                self.cur_tok(),
                Some(Token::DollarSign),
                |tok| parser_error!("Expected dollar sign, received {tok:?} instead")
            );
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

    fn parse_typed_ident(&mut self) -> Option<IRTypedIdent<'p>> {
        let ident = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => panic!("Expected ident for the name"),
        };
        self.next_tok();
        let _type = self.parse_type()?;
        Some(IRTypedIdent {
            ident: ident,
            _type,
        })
    }

    fn parse_lit(&mut self) -> Option<IRExpr<'p>> {
        // `l`
        self.next_tok();
        // `{`
        self.next_tok();
        let lit = match self.cur_tok()? {
            Token::LitString(string) => Literal::String(string.trim_matches('"')),
            Token::LitInt(int) => Literal::Int32(int.parse().unwrap()),
            Token::LitFloat(float) => Literal::Float32(float.parse().unwrap()),
            Token::LitChar(char) => Literal::Char(char.parse().unwrap()),
            _ => parser_error!("Expected literal after `l{{`"),
        };
        expect_tok!(self.peek_tok()?, Token::Colon, |tok| {
            parser_error!("Expected colon to seperate literal from type suffix, received {tok:?}");
        });
        self.next_tok();
        self.next_tok();
        let type_ = self.parse_type()?;
        expect_tok!(self.peek_tok()?, Token::RCurly, |tok| {
            parser_error!(
                "Expected right curly brackets after type suffix, received {tok:?} instead"
            );
        });
        self.next_tok();
        Some(IRExpr::Literal(lit, type_))
    }

    #[inline(always)]
    fn cur_tok(&self) -> Option<&Token<'p>> {
        self.lexer.tokens.get(self.tok_index)
    }

    #[inline(always)]
    fn peek_tok(&self) -> Option<&Token<'p>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    #[inline(always)]
    fn next_tok(&mut self) {
        self.tok_index += 1;
    }
}
