//! The compiler module is responsible for taking the AST and converting it into IR code.

use citadel_frontend::{api::IRCompiler, ir::{self, *}};

use super::ast::{self, BlockStatement, CallExpression, Expression, FnStatement, InfixOpExpr, LetStatement, ReturnStatement, Statement, TypedIdent};

#[derive(Default)]
pub struct Compiler;

impl Compiler {
    pub fn compile_program(&self, ast: Vec<Statement>) -> Vec<IRStmt> {
        let mut ir_stream = Vec::new();
        
        ir_stream.push(self.init_program());

        for stmt in ast {
            ir_stream.push(self.compile_stmt(stmt))
        }
        ir_stream
    }

    fn init_program(&self) -> IRStmt {
        IRStmt::Label(LabelStmt {
            name: "_entry".into(),
            block: BlockStmt {
                stmts: vec![IRStmt::Expression(IRExpr::Call(CallExpr {
                    name: "main".into(),
                    args: Vec::new(),
                }))],
            },
        })
    }

    fn compile_stmt(&self, stmt: Statement) -> IRStmt {
        match stmt {
            Statement::Let(_let) => self.compile_let_stmt(_let),
            Statement::Fn(_fn) => self.compile_fn_stmt(_fn),
            Statement::If(_if) => todo!(),
            Statement::Loop(_loop) => todo!(),
            Statement::Block(_block) => todo!(),
            Statement::Return(_ret) => self.compile_ret_stmt(_ret),
            Statement::Expression(_expr) => IRStmt::Expression(self.compile_expr(_expr)),
        }
    }

    fn compile_expr(&self, expr: Expression) -> IRExpr {
        match expr {
            Expression::Call(call) => self.compile_call_expr(call),
            Expression::Infix(op) => self.compile_arith_op_expr(op),
            Expression::Literal(lit) => self.compile_lit(lit),
        }
    }

    fn compile_let_stmt(&self, node: LetStatement) -> IRStmt {
        IRStmt::Variable(VarStmt {
            name: IRTypedIdent {
                ident: node.name.ident,
                _type: node.name._type,
            },
            is_const: true,
            val: self.compile_expr(node.val),
        })
    }

    fn compile_ret_stmt(&self, ret: ReturnStatement) -> IRStmt {
        IRStmt::Return(ReturnStmt {
            ret_val: self.compile_expr(ret.val),
        })
    }

    fn compile_fn_stmt(&self, node: FnStatement) -> IRStmt {
        IRStmt::Function(FuncStmt {
            block: {
                let mut block = self.compile_block_stmt(node.block).stmts;
                if node.name.as_str() == "main" {
                    if let Some(last) = block.last() {
                        match last {
                            IRStmt::Return(_) => (),
                            _ => block.push(IRStmt::Return(ReturnStmt {
                                ret_val: IRExpr::Literal(ir::Literal::Int32(0)),
                            })),
                        }
                    }
                }
                BlockStmt { stmts: block }
            },
            name: IRTypedIdent {
                _type: match node.name.as_str() {
                    "main" => "i32".into(),
                    _ => node.ret_type,
                },
                ident: node.name,
            },
            args: self.compile_def_args(node.args),
        })
    }

    fn compile_call_expr(&self, node: CallExpression) -> IRExpr {
        IRExpr::Call(CallExpr {
            name: if &node.name == "puts" {
                "print".into()
            } else {
                node.name
            },
            args: self.compile_args(node.args),
        })
    }

    fn compile_arith_op_expr(&self, node: InfixOpExpr) -> IRExpr {
        match node.operator {
            ast::Operator::Add | ast::Operator::Sub | ast::Operator::Mul | ast::Operator::Div => {
                IRExpr::ArithOp(ArithOpExpr {
                    op: self.compiler_op(node.operator),
                    values: self.compile_expr_tuple((*node.sides.0, *node.sides.1)),
                })
            }
            ast::Operator::Reassign => todo!(),
            ast::Operator::Equals => todo!(),
        }
    }

    fn compile_expr_tuple(&self, tuple: (Expression, Expression)) -> (Box<IRExpr>, Box<IRExpr>) {
        (
            Box::new(self.compile_expr(tuple.0)),
            Box::new(self.compile_expr(tuple.1)),
        )
    }

    fn compiler_op(&self, op: ast::Operator) -> ir::Operator {
        match op {
            ast::Operator::Add => ir::Operator::Add,
            ast::Operator::Sub => ir::Operator::Sub,
            ast::Operator::Mul => ir::Operator::Mul,
            ast::Operator::Div => ir::Operator::Div,
            ast::Operator::Reassign => todo!(),
            ast::Operator::Equals => todo!(),
        }
    }

    fn compile_args(&self, args: Vec<Expression>) -> Vec<IRExpr> {
        let mut arg_outs = Vec::new();
        for arg in args {
            arg_outs.push(self.compile_expr(arg))
        }
        arg_outs
    }

    fn compile_lit(&self, node: ast::Literal) -> IRExpr {
        match node {
            ast::Literal::Ident(ident) => IRExpr::Ident(ident),
            ast::Literal::String(string) => IRExpr::Literal(ir::Literal::String(string)),
            ast::Literal::Integer(int) => IRExpr::Literal(ir::Literal::Int32(int)),
            ast::Literal::Float(float) => IRExpr::Literal(ir::Literal::Double(float)),
            ast::Literal::Boolean(bool) => IRExpr::Literal(ir::Literal::Bool(bool)),
        }
    }

    fn compile_typed_ident(&self, node: TypedIdent) -> IRTypedIdent {
        IRTypedIdent {
            _type: node._type,
            ident: node.ident,
        }
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
