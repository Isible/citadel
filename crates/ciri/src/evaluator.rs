use frontend::ast::{IRStmt, IRExpr, Literal};

use crate::{env::Environment, obj::{Object, StrObj, CharObj, FloatObj, BoolObj, IntObj}};

pub(crate) struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub(crate) fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub(crate) fn eval_program(&mut self, program: Vec<IRStmt>) -> Option<Object> {
        let mut val = None;
        for node in program {
            val = Some(self.eval_stmt(node));
        }
        val
    }

    fn eval_stmt(&mut self, node: IRStmt) -> Object {
        match node {
            IRStmt::DeclaredFunction(func) => todo!(),
            IRStmt::Function(func) => todo!(),
            IRStmt::Variable(var) => todo!(),
            IRStmt::Constant(_const) => todo!(),
            IRStmt::Label(label) => todo!(),
            IRStmt::Return(ret) => todo!(),
            IRStmt::Break(br) => todo!(),
            IRStmt::Jump(goto) => todo!(),
            IRStmt::Call(call) => todo!(),
            IRStmt::Expression(expr) => self.eval_expr(expr),
        }
    }

    fn eval_expr(&mut self, node: IRExpr) -> Object {
        match node {
            IRExpr::Call(_) => todo!(),
            IRExpr::Literal(lit) => self.eval_lit(lit),
            IRExpr::Ident(_) => todo!(),
            IRExpr::ArithOp(_) => todo!(),
        }
    }

    fn eval_lit(&mut self, node: Literal) -> Object {
        match node {
            Literal::String(str) => Object::String(StrObj(str)),
            Literal::Char(char) => Object::Char(CharObj(char)),
            Literal::ShortFloat(_, float) => Object::Float(FloatObj(float)),
            Literal::LongFloat(_, float) => Object::Float(FloatObj(float as f32)),
            Literal::Bool(bool) => Object::Boolean(BoolObj(bool)),
            Literal::Integer(_, int) => Object::Integer(IntObj(int)),
            Literal::Array(_, arr) => todo!(),
            Literal::Vector(vec) => todo!(),
        }
    }    
}
