use std::thread::panicking;

use frontend::ast::{CallExpr, FuncStmt, IRExpr, IRStmt, LabelStmt, Literal, VarStmt};

use crate::{
    env::{EnvObj, EnvObjType, Environment}, obj::{FuncObj, LabelObj, Object}, parser::Parser
};

pub(crate) struct Evaluator {
    pub(crate) env: Environment,
}

impl Evaluator {
    pub(crate) fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub(crate) fn eval_program(&mut self, parser: &mut Parser) -> Option<Object> {
        let mut val = None;
        while let Some(stmt) = parser.parse_stmt() {
            val = self.eval_stmt(stmt);
            parser.next_token();
        }
        val
    }

    pub(crate) fn eval_stmt(&mut self, node: IRStmt) -> Option<Object> {
        match node {
            IRStmt::DeclaredFunction(func) => todo!(),
            IRStmt::Function(func) => self.eval_function(func),
            IRStmt::Variable(var) => self.eval_var(var),
            IRStmt::Label(label) => self.eval_label(label),
            IRStmt::Return(ret) => todo!(),
            IRStmt::Break(br) => todo!(),
            IRStmt::Jump(jmp) => todo!(),
            IRStmt::Call(call) => Some(self.eval_call(call)),
            IRStmt::Expression(expr) => Some(self.eval_expr(expr)),
        }
    }

    fn eval_expr(&mut self, node: IRExpr) -> Object {
        match node {
            IRExpr::Call(call) => self.eval_call(call),
            IRExpr::Literal(node) => Object::Literal(node),
            IRExpr::Ident(_) => todo!(),
            IRExpr::ArithOp(_) => todo!(),
        }
    }

    fn eval_function(&mut self, node: FuncStmt) -> Option<Object> {
        let obj = Object::FuncObj(FuncObj {
            args: node.args,
            block: node.block,
        });
        let (name, _type) = (node.name.ident, node.name._type);
        self.env.set(
            name,
            EnvObj {
                _type: EnvObjType::Function {
                    is_local: node.is_local,
                    ret_type: _type,
                },
                val: obj,
            },
        );
        None
    }

    fn eval_var(&mut self, node: VarStmt) -> Option<Object> {
        let val = self.eval_expr(node.val);
        if val == Object::None {
            panic!("The value of variable: `{}` is none", &node.name.ident);
        }
        self.env.set(
            node.name.ident,
            EnvObj {
                _type: EnvObjType::Variable {
                    is_const: node.is_const,
                    is_local: node.is_local,
                },
                val,
            },
        );
        None
    }

    fn eval_label(&mut self, node: LabelStmt) -> Option<Object> {
        self.env.set(
            node.name,
            EnvObj {
                _type: EnvObjType::Label,
                val: Object::Label(LabelObj { block: node.block }),
            },
        );
        None
    }

    fn eval_call(&mut self, node: CallExpr) -> Object {
        match node.name.as_str() {
            // debugging
            "print" => {
                println!("{:#?}", self.eval_expr(node.args.first().unwrap().clone()));
                Object::None
            }
            _ => todo!(),
        }
    }
}
