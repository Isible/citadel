//! The compiler module is responsible for taking the AST and converting it into IR code.
mod utils;

use std::mem;

use bumpalo::Bump;
use citadel_api::frontend::ir::{self, irgen::IRGenerator, IRExpr, IRStmt, IRTypedIdent, VarStmt};

use super::ast::{self, Ident, Type, *};
use utils as cutils;

#[derive(Default)]
pub struct Compiler<'c> {
    pub arena: Bump,
    pub out: IRGenerator<'c>,

    label_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileCtx<'ctx> {
    RetType(ir::Type<'ctx>),
    VarType(ir::Type<'ctx>),
}

impl<'ctx> CompileCtx<'ctx> {
    fn _type(&self) -> ir::Type<'ctx> {
        match self {
            CompileCtx::RetType(t) => *t,
            CompileCtx::VarType(t) => *t,
        }
    }
}

impl<'c> Compiler<'c> {
    pub fn compile_program(&mut self, ast: Vec<Statement<'c>>) {
        for node in ast {
            self.compile_stmt(node);
        }
    }

    fn compile_stmt(&mut self, node: Statement<'c>) {
        match node {
            Statement::Let(node) => self.compile_let_stmt(node),
            Statement::Fn(node) => self.compile_fn_stmt(node),
            Statement::If(_) => todo!(),
            Statement::Loop(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Block(_) => todo!(),
            Statement::Expression(_) => todo!(),
        }
    }

    fn compile_let_stmt(&mut self, node: LetStatement<'c>) {
        let name = cutils::compile_typed_ident(node.name);
        let val = self.compile_expr(node.val, Some(CompileCtx::VarType(name._type)));
        let stmt = IRStmt::Variable(VarStmt {
            val,
            is_const: true,
            name,
        });
        self.out.gen_ir(stmt);
    }

    fn compile_fn_stmt(&mut self, node: FnStatement<'c>) {
        let stmt = IRStmt::Function(ir::FuncStmt {
            name: cutils::compile_typed_ident(TypedIdent {
                _type: node.ret_type,
                ident: &node.name,
            }),
            args: vec![],
            block: self.compile_block_stmt(),
        });
        self.out.gen_ir(stmt);
    }

    fn compile_block_stmt(&mut self) -> ir::BlockStmt<'c> {
        let mut block: Vec<IRStmt<'c>> = Vec::new();
        mem::swap(self.out.mut_stream_ref().mut_stream_ref(), &mut block);
    }

    fn compile_expr(&self, node: Expression<'c>, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match node {
            Expression::Literal(node) => self.compile_lit_expr(node, ctx),
            _ => todo!(),
        }
    }

    fn compile_lit_expr(&self, node: Literal<'c>, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match node {
            Literal::Integer(int) => IRExpr::Literal(
                ir::Literal::Int32(int),
                match ctx {
                    Some(t) => t._type(),
                    None => ir::Type::Ident(ir::Ident("i32")),
                },
            ),
            _ => todo!(),
        }
    }

    /*
    pub fn compile_program(&'c mut self, ast: &'c Vec<Statement>) -> HIRStream<'c> {
        self.out.gen_ir(Self::init_program());

        for stmt in ast {
            self.compile_stmt(stmt, None)
        }
        self.out.stream()
    }
    */

    /*
    fn init_program() {
        IRStmt::Entry(BlockStmt {
            stmts: vec![IRStmt::Exit(ExitStmt {
                exit_code: IRExpr::Call(CallExpr {
                    name: Ident("main"),
                    args: Vec::new(),
                }),
            })],
        });
    }

    fn compile_stmt(&'c mut self, stmt: &'c Statement<'c>, ctx: Option<CompileCtx<'c>>) {
        match stmt {
            Statement::Let(node) => self.compile_let_stmt(node),
            Statement::Fn(node) => self.compile_fn_stmt(node),
            Statement::If(node) => todo!(),
            Statement::Loop(node) => self.compile_loop_stmt(node),
            Statement::Block(node) => todo!(),
            Statement::Return(node) => self.compile_ret_stmt(node, ctx),
            Statement::Expression(Expression::Call(node)) => match node.name {
                ast::Ident::Slice("exit") => self.compile_exit_call(node, ctx),
                _ => {IRStmt::Call(self.compile_call_expr(node, ctx));},
            },
            _ => panic!(),
        }
    }

    fn compile_expr(&'c mut self, expr: &'c Expression<'c>, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match expr {
            Expression::Call(call) => IRExpr::Call(self.compile_call_expr(call, ctx)),
            Expression::Infix(op) => self.compile_arith_op_expr(op, ctx),
            Expression::Literal(lit) => self.compile_lit(lit, ctx),
        }
    }

    fn compile_let_stmt(&'c mut self, node: &'c LetStatement<'c>) {
        IRStmt::Variable(VarStmt {
            name: self.compile_typed_ident(&node.name),
            is_const: true,
            val: self.compile_expr(&node.val, Some(CompileCtx::VarType(&node.name._type))),
        });
    }

    fn compile_ret_stmt(
        &'c mut self,
        ret: &'c ReturnStatement,
        ctx: Option<CompileCtx<'c>>,
    ) {
        IRStmt::Return(ReturnStmt {
            ret_val: self.compile_expr(&ret.val, ctx),
        });
    }

    fn compile_fn_stmt(&'c mut self, node: &'c FnStatement) {
        IRStmt::Function(FuncStmt {
            block: {
                let mut block = self
                    .compile_block_stmt(&node.block, Some(CompileCtx::RetType(&node.ret_type)))
                    .stmts;
                if let ast::Ident::Slice("main") = node.name {
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
                    ast::Ident::Slice("main") => ir::Type::Ident(Ident("i32")),
                    _ => self.compile_type(&node.ret_type),
                },
                ident: ir::Ident(match node.name {
                    ast::Ident::Slice(s) => s,
                    ast::Ident::Owned(_) => todo!(),
                }),
            },
            args: self.compile_def_args(&node.args),
        });
    }

    fn compile_loop_stmt(&'c self, node: &'c LoopStatement) {
        IRStmt::Label(LabelStmt { name: Ident("L") });
    }

    fn compile_call_expr(
        &self,
        node: &'c CallExpression,
        ctx: Option<CompileCtx<'c>>,
    ) -> CallExpr<'c> {
        CallExpr {
            name: match node.name {
                ast::Ident::Slice("puts") => Ident("print"),
                _ => ir::Ident(match node.name {
                    ast::Ident::Slice(s) => s,
                    ast::Ident::Owned(_) => todo!(),
                }),
            },
            args: self.compile_args(&node.args),
        }
    }

    fn compile_exit_call(
        &self,
        node: &'c CallExpression,
        ctx: Option<CompileCtx<'c>>,
    ) {
        let expr = node
            .args
            .first()
            .unwrap_or_else(|| panic!("Expected exit call to have one argument for the exit code"));
        IRStmt::Exit(ExitStmt {
            exit_code: self.compile_expr(expr, ctx),
        });
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
                    values: self.compile_expr_tuple((node.sides.0, node.sides.1), ctx),
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
            ast::Literal::Ident(ident) => IRExpr::Ident(ir::Ident(match ident {
                ast::Ident::Slice(s) => s,
                ast::Ident::Owned(_) => todo!(),
            })),
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

    fn compile_typed_ident(&'c self, node: &'c TypedIdent<'c>) -> IRTypedIdent<'c> {
        IRTypedIdent {
            _type: self.compile_type(&node._type),
            ident: Ident(match node.ident {
                ast::Ident::Slice(s) => s,
                ast::Ident::Owned(_) => todo!(),
            }),
        }
    }

    fn compile_def_args(&'c self, node: &'c Vec<TypedIdent<'c>>) -> Vec<IRTypedIdent<'c>> {
        let mut out = Vec::new();
        for node in node {
            out.push(self.compile_typed_ident(node))
        }
        out
    }

    fn compile_block_stmt(
        &'c mut self,
        node: &'c BlockStatement<'c>,
        ctx: Option<CompileCtx<'c>>,
    ) -> BlockStmt<'c> {
        let start = self.out.stream_ref().stream.len();
        for node in &node.stmts {
            self.compile_stmt(node, ctx)
        }
        let out = &self.out.stream_ref().stream.as_slice()[start..];
        BlockStmt { stmts: Vec::from(out) }
    }

    fn compile_type(&'c self, _type: &'c ast::Type<'c>) -> ir::Type<'c> {
        match _type {
            ast::Type::Ident(ident) => match ident {
                ast::Ident::Slice("int") => ir::Type::Ident(ir::Ident("i32")),
                ast::Ident::Slice("float") => ir::Type::Ident(ir::Ident("f32")),
                ast::Ident::Slice("string") => ir::Type::Array(&ir::Type::Ident(Ident("i8")), 8),
                id => panic!("{id:?}"),
            },
            ast::Type::Array(_type, len) => {
                let _type = self.arena.alloc(self.compile_type(_type));
                ir::Type::Array(_type, *len)
            }
        }
    }
    */
}
