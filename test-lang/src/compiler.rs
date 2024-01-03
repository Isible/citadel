use frontend::{ast::*, ir_gen::IRGenerator};

use crate::{
    ast::{
        ArithmeticOperationExpr, CallExpression, Expression, FnStatement, LetStatement, Literal,
        Operator, Statement, TypedIdent, BlockStatement,
    },
    parser::{EofError, Parser},
};

pub struct Compiler<'a> {
    pub generator: IRGenerator,
    pub parser: &'a mut Parser<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Result<Self, EofError> {
        let mut compiler = Self {
            generator: IRGenerator::new(),
            parser,
        };

        compiler.init_program();

        Ok(compiler)
    }

    pub fn compile_program(&mut self) {
        loop {
            let stmt = match self.parser.parse_stmt() {
                Ok(stmt) => stmt,
                Err(_) => break,
            };
            let node = self.compile_stmt(stmt);
            self.generator.gen_ir(node);
        }
    }

    fn init_program(&mut self) {
        self.generator
            .gen_ir(frontend::ast::IRStmt::Label(LabelStmt {
                name: "entry".into(),
                block: BlockStmt {
                    stmts: vec![frontend::ast::IRStmt::Expression(
                        frontend::ast::IRExpr::Call(CallExpr {
                            name: "main".into(),
                            args: vec![frontend::ast::IRExpr::Call(CallExpr {
                                name: "citadel.std.env.args".into(),
                                args: Vec::new(),
                            })],
                        }),
                    )],
                },
            }))
    }

    fn compile_stmt(&self, stmt: Statement) -> IRStmt {
        match stmt {
            Statement::Let(_let) => self.compile_let_stmt(_let),
            Statement::Fn(_fn) => self.compile_fn_stmt(_fn),
            Statement::If(_if) => todo!(),
            Statement::Loop(_loop) => todo!(),
            Statement::Call(_call) => IRStmt::Expression(self.compile_call_expr(_call)),
            Statement::Block(_block) => todo!(),
        }
    }

    fn compile_expr(&self, expr: Expression) -> IRExpr {
        match expr {
            Expression::Call(call) => self.compile_call_expr(call),
            Expression::ArithmeticOperation(op) => self.compile_arith_op_expr(op),
            Expression::Literal(lit) => self.compile_lit(lit),
        }
    }

    fn compile_let_stmt(&self, node: LetStatement) -> IRStmt {
        IRStmt::Constant(ConstStmt {
            name: IRTypedIdent {
                ident: node.name.ident,
                _type: node.name._type,
            },
            is_local: true,
            val: self.compile_expr(node.val),
        })
    }

    fn compile_fn_stmt(&self, node: FnStatement) -> IRStmt {
        IRStmt::Function(FuncStmt {
            name: IRTypedIdent {
                ident: node.name,
                _type: node.ret_type,
            },
            args: self.compile_def_args(node.args),
            is_local: true,
            block: self.compile_block_stmt(node.block),
        })
    }

    fn compile_call_expr(&self, node: CallExpression) -> IRExpr {
        IRExpr::Call(CallExpr {
            name: node.name,
            args: self.compile_args(node.args),
        })
    }

    fn compile_arith_op_expr(&self, node: ArithmeticOperationExpr) -> IRExpr {
        match node.operator {
            Operator::Add => IRExpr::Add(AddExpr {
                values: (
                    Box::new(self.compile_expr(*node.sides.0)),
                    Box::new(self.compile_expr(*node.sides.1)),
                ),
            }),
            Operator::Sub => IRExpr::Sub(SubExpr {
                values: (
                    Box::new(self.compile_expr(*node.sides.0)),
                    Box::new(self.compile_expr(*node.sides.1)),
                ),
            }),
            Operator::Div => IRExpr::Div(DivExpr {
                values: (
                    Box::new(self.compile_expr(*node.sides.0)),
                    Box::new(self.compile_expr(*node.sides.1)),
                ),
            }),
            Operator::Multiply => IRExpr::Mul(MulExpr {
                values: (
                    Box::new(self.compile_expr(*node.sides.0)),
                    Box::new(self.compile_expr(*node.sides.1)),
                ),
            }),
            Operator::Reassign => todo!(),
            Operator::Equals => todo!(),
        }
    }

    fn compile_args(&self, args: Vec<Expression>) -> Vec<IRExpr> {
        let mut arg_outs = Vec::new();
        for arg in args {
            arg_outs.push(self.compile_expr(arg))
        }
        arg_outs
    }

    fn compile_lit(&self, node: Literal) -> IRExpr {
        match node {
            Literal::Variable(_) => todo!(),
            Literal::String(string) => IRExpr::Literal(frontend::ast::Literal::String(string)),
            Literal::Integer(int) => IRExpr::Literal(frontend::ast::Literal::Integer(
                /* TODO: change this accordingly */ 32,
                int as isize,
            )),
            Literal::Float(float) => IRExpr::Literal(frontend::ast::Literal::LongFloat(64, float)),
            Literal::Boolean(bool) => IRExpr::Literal(frontend::ast::Literal::Bool(bool)),
        }
    }

    fn compile_typed_ident(&self, node: TypedIdent) -> IRTypedIdent {
        IRTypedIdent { _type: node._type, ident: node.ident }
    }

    fn compile_def_args(&self, node: Vec<TypedIdent>) -> Vec<IRTypedIdent> {
        let mut out = Vec::new();
        for node in node {
            out.push(self.compile_typed_ident(node))
        }
        out
    }

    fn compile_block_stmt(&self, node: BlockStatement) -> BlockStmt {
        let mut out = Vec::new();
        for node in node.stmts {
            out.push(self.compile_stmt(node))
        }
        BlockStmt { stmts: out }
    }
}
