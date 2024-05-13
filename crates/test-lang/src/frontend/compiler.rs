//! The compiler module is responsible for taking the AST and converting it into IR code.

use citadel_api::frontend::ir::{
    self,
    irgen::{IRGenerator, IRStream},
    *,
};

use super::ast::{self, *};

#[derive(Default)]
pub struct Compiler;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileCtx<'ctx> {
    RetType(&'ctx ast::Type<'ctx>),
    VarType(&'ctx ast::Type<'ctx>),
}

impl<'c> Compiler {
    pub fn compile_program(&self, ast: &'c Vec<Statement>) -> IRStream<'c> {
        let mut ir_gen = IRGenerator::default();

        ir_gen.gen_ir(Self::init_program());

        for stmt in ast {
            ir_gen.gen_ir(self.compile_stmt(stmt, None))
        }
        ir_gen.stream()
    }

    fn init_program() -> IRStmt<'c> {
        IRStmt::Label(LabelStmt {
            name: Ident("_entry"),
            block: BlockStmt {
                stmts: vec![IRStmt::Exit(ExitStmt {
                    exit_code: IRExpr::Call(CallExpr {
                        name: Ident("main"),
                        args: Vec::new(),
                    }),
                })],
            },
        })
    }

    fn compile_stmt(&self, stmt: &'c Statement<'c>, ctx: Option<CompileCtx<'c>>) -> IRStmt<'c> {
        match stmt {
            Statement::Let(node) => self.compile_let_stmt(node),
            Statement::Fn(node) => self.compile_fn_stmt(node),
            Statement::If(node) => todo!(),
            Statement::Loop(node) => todo!(),
            Statement::Block(node) => todo!(),
            Statement::Return(node) => self.compile_ret_stmt(node, ctx),
            Statement::Expression(Expression::Call(node)) => match node.name {
                "exit" => self.compile_exit_call(node, ctx),
                _ => IRStmt::Call(self.compile_call_expr(node, ctx)),
            },
            _ => panic!(),
        }
    }

    fn compile_expr(&self, expr: &'c Expression<'c>, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match expr {
            Expression::Call(call) => IRExpr::Call(self.compile_call_expr(call, ctx)),
            Expression::Infix(op) => self.compile_arith_op_expr(op, ctx),
            Expression::Literal(lit) => self.compile_lit(lit, ctx),
        }
    }

    fn compile_let_stmt(&self, node: &'c LetStatement<'c>) -> IRStmt<'c> {
        IRStmt::Variable(VarStmt {
            name: self.compile_typed_ident(&node.name),
            is_const: true,
            val: self.compile_expr(&node.val, Some(CompileCtx::VarType(&node.name._type))),
        })
    }

    fn compile_ret_stmt(
        &self,
        ret: &'c ReturnStatement,
        ctx: Option<CompileCtx<'c>>,
    ) -> IRStmt<'c> {
        IRStmt::Return(ReturnStmt {
            ret_val: self.compile_expr(&ret.val, ctx),
        })
    }

    fn compile_fn_stmt(&self, node: &'c FnStatement) -> IRStmt<'c> {
        IRStmt::Function(FuncStmt {
            block: {
                let mut block = self
                    .compile_block_stmt(&node.block, Some(CompileCtx::RetType(&node.ret_type)))
                    .stmts;
                if node.name == "main" {
                    if let Some(last) = block.last() {
                        match last {
                            IRStmt::Return(_) => (),
                            _ => block.push(IRStmt::Return(ReturnStmt {
                                ret_val: IRExpr::Literal(
                                    ir::Literal::Int32(0),
                                    ir::Type::Ident(ir::Ident("i32")),
                                ),
                            })),
                        }
                    }
                }
                BlockStmt { stmts: block }
            },
            name: IRTypedIdent {
                _type: match node.name {
                    "main" => ir::Type::Ident(Ident("i32")),
                    _ => ir::Type::Ident(Self::compile_type(&node.ret_type)),
                },
                ident: Ident(&node.name),
            },
            args: self.compile_def_args(&node.args),
        })
    }

    fn compile_call_expr(
        &self,
        node: &'c CallExpression,
        ctx: Option<CompileCtx<'c>>,
    ) -> CallExpr<'c> {
        CallExpr {
            name: match node.name {
                "puts" => Ident("print"),
                _ => Ident(&node.name),
            },
            args: self.compile_args(&node.args),
        }
    }

    fn compile_exit_call(
        &self,
        node: &'c CallExpression,
        ctx: Option<CompileCtx<'c>>,
    ) -> IRStmt<'c> {
        let expr = node
            .args
            .get(0)
            .unwrap_or_else(|| panic!("Expected exit call to have one argument for the exit code"));
        IRStmt::Exit(ExitStmt {
            exit_code: self.compile_expr(expr, ctx),
        })
    }

    fn compile_arith_op_expr(
        &self,
        node: &'c InfixOpExpr,
        ctx: Option<CompileCtx<'c>>,
    ) -> IRExpr<'c> {
        match node.operator {
            ast::Operator::Add | ast::Operator::Sub | ast::Operator::Mul | ast::Operator::Div => {
                IRExpr::ArithOp(ArithOpExpr {
                    op: self.compile_op(node.operator),
                    values: self.compile_expr_tuple((&*node.sides.0, &*node.sides.1), ctx),
                })
            }
            ast::Operator::Reassign => todo!(),
            ast::Operator::Equals => todo!(),
        }
    }

    fn compile_expr_tuple(
        &self,
        tuple: (&'c Expression, &'c Expression),
        ctx: Option<CompileCtx<'c>>,
    ) -> (Box<IRExpr<'c>>, Box<IRExpr<'c>>) {
        (
            Box::new(self.compile_expr(tuple.0, ctx)),
            Box::new(self.compile_expr(tuple.1, ctx)),
        )
    }

    fn compile_op(&self, op: ast::Operator) -> ir::Operator {
        match op {
            ast::Operator::Add => ir::Operator::Add,
            ast::Operator::Sub => ir::Operator::Sub,
            ast::Operator::Mul => ir::Operator::Mul,
            ast::Operator::Div => ir::Operator::Div,
            ast::Operator::Reassign => todo!(),
            ast::Operator::Equals => todo!(),
        }
    }

    fn compile_args(&self, args: &'c Vec<Expression>) -> Vec<IRExpr<'c>> {
        let mut arg_outs = Vec::new();
        for arg in args {
            arg_outs.push(self.compile_expr(arg, None))
        }
        arg_outs
    }

    fn compile_lit(&self, node: &'c ast::Literal, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match node {
            ast::Literal::Ident(ident) => IRExpr::Ident(Ident(&ident)),
            ast::Literal::String(string) => IRExpr::Literal(
                ir::Literal::String(string),
                ir::Type::Array(&ir::Type::Ident(Ident("i8")), string.len()),
            ),
            ast::Literal::Integer(int) => {
                IRExpr::Literal(ir::Literal::Int32(*int), ir::Type::Ident(ir::Ident("i32")))
            }
            ast::Literal::Float(float) => IRExpr::Literal(
                ir::Literal::Double(*float),
                ir::Type::Ident(ir::Ident("f32")),
            ),
            ast::Literal::Boolean(bool) => {
                IRExpr::Literal(ir::Literal::Bool(*bool), ir::Type::Ident(ir::Ident("i8")))
            }
        }
    }

    fn compile_typed_ident(&self, node: &'c TypedIdent<'c>) -> IRTypedIdent<'c> {
        IRTypedIdent {
            _type: ir::Type::Ident(Self::compile_type(&node._type)),
            ident: Ident(&node.ident),
        }
    }

    fn compile_def_args(&self, node: &'c Vec<TypedIdent<'c>>) -> Vec<IRTypedIdent<'c>> {
        let mut out = Vec::new();
        for node in node {
            out.push(self.compile_typed_ident(&node))
        }
        out
    }

    fn compile_block_stmt(
        &self,
        node: &'c BlockStatement<'c>,
        ctx: Option<CompileCtx<'c>>,
    ) -> BlockStmt<'c> {
        let mut out = Vec::new();
        for node in &node.stmts {
            out.push(self.compile_stmt(node, ctx))
        }
        BlockStmt { stmts: out }
    }

    fn compile_type(_type: &'c ast::Type<'c>) -> ir::Ident<'c> {
        ir::Ident(match _type {
            ast::Type::Ident(ident) => match *ident {
                "int" => "i32",
                "float" => "f32",
                _ => panic!(),
            },
            ast::Type::Array(_, _) => todo!(),
        })
    }
}
