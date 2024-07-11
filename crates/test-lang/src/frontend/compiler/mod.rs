//! The compiler module is responsible for taking the AST and converting it into IR code.
use std::{mem, vec};

use bumpalo::Bump;
use citadel_api::frontend::ir::{
    self,
    irgen::{HIRStream, IRGenerator},
    CallExpr, ExitStmt, IRExpr, IRStmt, IRTypedIdent, VarStmt, FLOAT64_T, INT32_T, INT8_T,
};

use super::ast::{self, *};

pub struct Compiler<'c> {
    arena: &'c Bump,
    out: IRGenerator<'c>,
    functions: ast::FunctionTable<'c>,

    label_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileCtx<'ctx> {
    RetType(ir::Type<'ctx>),
    VarType(ir::Type<'ctx>),
}

macro_rules! no_ctx {
    ($expr:expr) => {{
        ($expr, None)
    }};
}

impl<'ctx> CompileCtx<'ctx> {
    fn as_type(&self) -> ir::Type<'ctx> {
        match self {
            CompileCtx::RetType(t) => *t,
            CompileCtx::VarType(t) => *t,
        }
    }
}

impl<'c> Compiler<'c> {
    pub fn compile_program(
        ast: Vec<Statement<'c>>,
        functions: FunctionTable<'c>,
        arena: &'c Bump,
    ) -> HIRStream<'c> {
        if !functions.contains_key("main") {
            panic!("Program does not have a main function");
        }

        let mut compiler = Compiler {
            functions,
            arena,
            label_index: Default::default(),
            out: Default::default(),
        };

        compiler.init_program();
        for node in ast {
            compiler.compile_stmt(node);
        }
        compiler.out.stream()
    }

    fn init_program(&mut self) {
        self.out.gen_ir(IRStmt::Entry(ir::BlockStmt {
            stmts: vec![IRStmt::Exit(ir::ExitStmt {
                exit_code: IRExpr::Call(CallExpr {
                    name: "main",
                    args: vec![],
                }),
            })],
        }))
    }

    fn compile_stmt(&mut self, node: Statement<'c>) {
        match node {
            Statement::Let(node) => self.compile_let_stmt(node),
            Statement::Fn(node) => self.compile_fn_stmt(node),
            Statement::If(_) => todo!(),
            Statement::Loop(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Block(_) => todo!(),
            Statement::Expression(node) => {
                if let Some(call) = self.compile_expr_stmt(node) {
                    self.out.gen_ir(call);
                }
            }
        }
    }

    fn compile_let_stmt(&mut self, node: LetStatement<'c>) {
        let name = self.compile_typed_ident(node.name);
        let (val, _) = self.compile_expr(node.val, Some(CompileCtx::VarType(name._type)));
        let stmt = IRStmt::Variable(VarStmt {
            val,
            is_const: true,
            name,
        });
        self.out.gen_ir(stmt);
    }

    fn compile_fn_stmt(&mut self, node: FnStatement<'c>) {
        let stmt = IRStmt::Function(ir::FuncStmt {
            name: self.compile_typed_ident(TypedIdent {
                _type: node.ret_type,
                ident: &node.name,
            }),
            args: self.compile_typed_idents(node.args),
            block: self.compile_block_stmt(node.block),
        });
        self.out.gen_ir(stmt);
    }

    fn compile_block_stmt(&mut self, node: BlockStatement<'c>) -> ir::BlockStmt<'c> {
        let mut block = Vec::new();
        mem::swap(self.out.mut_stream_ref().mut_stream_ref(), &mut block);
        for stmt in node.stmts {
            self.compile_stmt(stmt);
        }
        mem::swap(self.out.mut_stream_ref().mut_stream_ref(), &mut block);
        ir::BlockStmt { stmts: block }
    }

    fn compile_expr_stmt(&mut self, node: Expression<'c>) -> Option<IRStmt<'c>> {
        match node {
            Expression::Call(node) => Some(ir::IRStmt::Call(self.compile_call_expr(node).0)),
            Expression::Infix(_) => None,
            Expression::Literal(_) => None,
        }
    }

    fn compile_expr(
        &mut self,
        node: Expression<'c>,
        ctx: Option<CompileCtx<'c>>,
    ) -> (IRExpr<'c>, Option<CompileCtx<'c>>) {
        match node {
            Expression::Literal(node) => no_ctx!(self.compile_lit_expr(node, ctx)),
            Expression::Call(node) => {
                if node.name == "exit" {
                    return self.compile_exit_expr(node);
                }
                let (call, ctx) = self.compile_call_expr(node);
                (IRExpr::Call(call), ctx)
            }
            Expression::Infix(_) => todo!(),
        }
    }

    fn compile_lit_expr(&self, node: Literal<'c>, ctx: Option<CompileCtx<'c>>) -> IRExpr<'c> {
        match node {
            Literal::Integer(int) => IRExpr::Literal(
                ir::Literal::Int32(int),
                ctx.map(|c| c.as_type()).unwrap_or(ir::Type::Ident(INT32_T)),
            ),
            Literal::Float(float) => IRExpr::Literal(
                ir::Literal::Float64(float),
                ctx.map(|c| c.as_type())
                    .unwrap_or(ir::Type::Ident(FLOAT64_T)),
            ),
            Literal::String(str) => IRExpr::Literal(
                ir::Literal::String(str),
                ctx.map(|c| c.as_type())
                    .unwrap_or(ir::Type::Array(&ir::Type::Ident(INT8_T), str.len() as u32)),
            ),
            Literal::Boolean(bool) => {
                IRExpr::Literal(ir::Literal::Bool(bool), ir::Type::Ident(INT8_T))
            }
            Literal::Char(ch) => {
                IRExpr::Literal(ir::Literal::Char(ch as u8), ir::Type::Ident(INT8_T))
            }
            Literal::Ident(ident) => IRExpr::Ident(ident),
        }
    }

    fn compile_call_expr(
        &mut self,
        node: CallExpression<'c>,
    ) -> (CallExpr<'c>, Option<CompileCtx<'c>>) {
        match node.name {
            "puts" => return self.compile_print_call(node),
            _ => (),
        }

        let func_info = self
            .functions
            .get(node.name)
            .expect(format!("No method with name: {}", node.name).as_str())
            .clone();
        let ctx = Some(CompileCtx::RetType(self.compile_type(func_info.ret_type)));
        let expr = ir::CallExpr {
            name: func_info.ir_name,
            args: self.compile_call_args(node.args, func_info.args.as_slice()),
        };
        (expr, ctx)
    }

    fn compile_call_args(
        &mut self,
        args: Vec<Expression<'c>>,
        args_info: &[TypedIdent<'c>],
    ) -> Vec<IRExpr<'c>> {
        let mut call_args = Vec::new();
        for (arg, info) in args.into_iter().zip(args_info) {
            let ctx = Some(CompileCtx::VarType(self.compile_type(info._type)));
            call_args.push(self.compile_expr(arg, ctx).0)
        }
        call_args
    }

    fn compile_type(&self, _type: Type<'c>) -> ir::Type<'c> {
        match _type {
            Type::Ident(id) => ir::Type::Ident(id),
            Type::Array(_type, len) => {
                let _type = self.arena.alloc(self.compile_type(*_type));
                ir::Type::Array(_type, len as u32)
            }
        }
    }

    fn compile_typed_ident(&self, typed_ident: TypedIdent<'c>) -> IRTypedIdent<'c> {
        IRTypedIdent {
            ident: typed_ident.ident,
            _type: self.compile_type(typed_ident._type),
        }
    }

    fn compile_typed_idents(&self, typed_idents: Vec<TypedIdent<'c>>) -> Vec<IRTypedIdent<'c>> {
        let mut ir_typed_idents = Vec::new();
        for typed_ident in typed_idents {
            ir_typed_idents.push(self.compile_typed_ident(typed_ident))
        }
        ir_typed_idents
    }

    fn compile_print_call(
        &mut self,
        node: CallExpression<'c>,
    ) -> (ir::CallExpr<'c>, Option<CompileCtx<'c>>) {
        let msg_expr = node
            .args
            .first()
            .expect("\"Puts\" function needs exactly one argument, currently it has none");
        let msg_len = match msg_expr {
            Expression::Literal(Literal::String(str)) => str.len(),
            _ => todo!(),
        };
        let expr = ir::CallExpr {
            name: "print",
            args: self.compile_call_args(
                node.args,
                &[TypedIdent {
                    _type: Type::Array(&Type::Ident(INT8_T), msg_len),
                    ident: "msg",
                }],
            ),
        };
        (expr, Some(CompileCtx::RetType(ir::Type::Ident("void"))))
    }

    fn compile_exit_expr(&mut self, mut node: CallExpression<'c>) -> (IRExpr<'c>, Option<CompileCtx<'c>>) {
        let exit_code = self.compile_expr(node.args.remove(0), Some(CompileCtx::VarType(ir::Type::Ident(INT32_T)))).0;
        self.out.gen_ir(IRStmt::Exit(ExitStmt {
            exit_code
        }));
        (IRExpr::Literal(ir::Literal::Int32(-1), ir::Type::Ident(INT32_T)), Some(CompileCtx::RetType(ir::Type::Ident(INT32_T))))
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
}
    */
