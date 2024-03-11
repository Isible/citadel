//! The evaluator module is responsible for evaluating/executing the IR nodes (AST).

use citadel_frontend::ir::{
    ArithOpExpr, CallExpr, FuncStmt, IRExpr, IRStmt, LabelStmt, Literal, Operator, VarStmt,
};
use citadel_irparser::parser::Parser;

use crate::{
    env::{EnvObj, EnvObjType, Environment},
    obj::{FuncObj, LabelObj, Object},
};

pub(crate) struct Evaluator<'a> {
    pub(crate) parser: &'a mut Parser<'a>,
    pub(crate) program: Vec<IRStmt>,
    pub(crate) env: Environment,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn new(parser: &'a mut Parser<'a>) -> Self {
        Self {
            program: parser.parse_program(),
            env: Environment::new(),
            parser,
        }
    }

    pub(crate) fn eval_program(&mut self) {
        let table = &self.parser.symbols;
        let entry = match table.get("_entry").unwrap() {
            IRStmt::Label(label) => &label.block.stmts,
            _ => panic!("The entry point is not a label"),
        };
        for stmt in entry.clone() {
            self.eval_stmt(stmt);
        }
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
            IRExpr::Ident(ident) => self.eval_ident(ident),
            IRExpr::ArithOp(op) => self.eval_arith_op(op),
        }
    }

    fn eval_ident(&mut self, ident: String) -> Object {
        let obj = self
            .env
            .get(&ident)
            .unwrap_or_else(|_| panic!("Variable: `{}` not found", &ident));
        obj.val
    }

    fn eval_arith_op(&mut self, op: ArithOpExpr) -> Object {
        let left = self.eval_expr(*op.values.0);
        let right = self.eval_expr(*op.values.1);
        let (left, right) = match (left, right) {
            (Object::Literal(Literal::Int64(left)), Object::Literal(Literal::Int64(right))) => {
                (left, right)
            }
            object => panic!("Invalid operands: `{:#?}` and `{:#?}`", object.0, object.1),
        };
        Object::Literal(Literal::Int64(match op.op {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        }))
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
                val: obj.clone(),
            },
        );
        Some(obj)
    }

    fn eval_var(&mut self, node: VarStmt) -> Option<Object> {
        let val = self.eval_expr(node.val);
        if val == Object::Void {
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
                Object::Void
            }
            name => {
                let func = self
                    .parser
                    .symbols
                    .get(name)
                    .unwrap_or_else(|| panic!("Function: `{}` not found", name));
                let func = match func {
                    IRStmt::Function(func) => func,
                    _ => todo!(),
                };
                for stmt in func.block.stmts.clone() {
                    self.eval_stmt(stmt);
                }
                Object::Void
            }
        }
    }
}
