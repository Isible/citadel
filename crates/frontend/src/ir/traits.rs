//! This file contains trait implementations for the IR node and utility structs for the frontend ir representation.
use crate::util::VecDisplay;
use std::fmt::Display;

use super::*;

impl Display for DeclFuncStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "decl func @{}({}) {}",
            self.name.ident,
            self.args.to_string(),
            self.name._type
        )
    }
}

impl Display for FuncStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "func @{}({}) {} {{\n{}\n}}",
            self.name.ident,
            self.args.to_string(),
            self.name._type,
            self.block
        )
    }
}

impl Display for VarStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {} = {}",
            if self.is_const { '$' } else { '?' },
            self.name.ident,
            self.name._type,
            self.val
        )
    }
}

impl Display for StructStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "struct @{} {{\n{}\n}}",
            self.name,
            self.fields.to_string()
        )
    }
}

impl Display for UnionStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "union @{} {{\n{}\n}}",
            self.name,
            self.variants.to_string()
        )
    }
}

impl Display for LabelStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}:", self.name)
    }
}

impl Display for ReturnStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ret {}", self.ret_val)
    }
}

impl Display for JumpStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jmp {}", self.label)
    }
}

impl Display for BlockStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmts = Vec::new();
        for stmt in &self.stmts {
            stmts.push("    ".into());
            stmts.push(stmt.to_string());
            stmts.push("\n".into());
        }
        stmts.pop();
        write!(f, "{}", stmts.join(""))
    }
}

impl Display for CallExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "call %{}({})", self.name, self.args.to_string())
    }
}

impl Display for ArithOpExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}",
            match self.op {
                Operator::Add => "add",
                Operator::Sub => "sub",
                Operator::Mul => "mul",
                Operator::Div => "div",
            },
            self.values.0,
            self.values.1
        )
    }
}

impl Display for IRStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            IRStmt::DeclaredFunction(func) => func.to_string(),
            IRStmt::Function(func) => func.to_string(),
            IRStmt::Variable(var) => var.to_string(),
            IRStmt::Label(label) => label.to_string(),
            IRStmt::Return(ret) => ret.to_string(),
            IRStmt::Exit(exit) => exit.to_string(),
            IRStmt::Jump(jump) => jump.to_string(),
            IRStmt::Call(call) => call.to_string(),
            IRStmt::Struct(_struct) => _struct.to_string(),
            IRStmt::Union(union) => union.to_string(),
            IRStmt::Entry(entry) => return entry_to_string(f, entry),
        })
    }
}

fn entry_to_string(f: &mut std::fmt::Formatter<'_>, entry: &BlockStmt<'_>) -> std::fmt::Result {
    write!(f, "entry {{\n {} \n}}", entry)
}

impl Display for IRExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            IRExpr::Call(call) => call.to_string(),
            IRExpr::Literal(lit, _type) => format!("l{{{}:{}}}", lit, _type),
            IRExpr::ArithOp(op) => op.to_string(),
            IRExpr::Ident(id) => id.to_string(),
            IRExpr::StructInit(init) => init.to_string(),
        })
    }
}

impl Display for StructInitExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct %{} {{{}}}", self.name, self.values.to_string())
    }
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &match self {
                Literal::String(string) => format!("\"{}\"", string),
                Literal::Char(char) => char.to_string(),
                Literal::Float32(val) => val.to_string(),
                Literal::Float64(val) => val.to_string(),
                Literal::Bool(val) => val.to_string(),
                Literal::Int8(val) => val.to_string(),
                Literal::Int16(val) => val.to_string(),
                Literal::Int32(val) => val.to_string(),
                Literal::Int64(val) => val.to_string(),
                Literal::Int128(val) => val.to_string(),
                Literal::Array(len, val) => format!("[{}; {}]", val.to_string(), len),
                Literal::Vector(val) => format!("<{}>", val.to_string()),
            }
        )
    }
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Ident(ident) => write!(f, "{ident}"),
            Type::Array(_type, size) => write!(f, "[{_type}; {size}]"),
        }
    }
}

impl Display for ExitStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exit {}", self.exit_code)
    }
}
