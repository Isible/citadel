use crate::util::vec_display::VecDisplay;
use std::fmt::Display;

use super::*;

impl Display for DeclFuncStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "decl @{}({}) {} {}",
            self.name.ident,
            self.args.to_string(),
            if self.is_local { "lcl" } else { "pub" },
            self.name._type
        )
    }
}

impl Display for FuncStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@{}({}) {} {} {{\n{}\n}}",
            self.name.ident,
            self.args.to_string(),
            if self.is_local { "lcl" } else { "pub" },
            self.name._type,
            self.block
        )
    }
}

impl Display for VarStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "?{} {} {} = {}",
            self.name,
            if self.is_local { "lcl" } else { "pub" },
            self.name._type,
            self.val
        )
    }
}

impl Display for ConstStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "${} {} {} = {}",
            self.name.ident,
            if self.is_local { "lcl" } else { "pub" },
            self.name._type,
            self.val
        )
    }
}

impl Display for LabelStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}: {{\n{}\n}}", self.name, self.block)
    }
}

impl Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ret {}", self.ret_val)
    }
}

impl Display for BreakStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "br {}", self.label)
    }
}

impl Display for JumpStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jmp {}", self.label)
    }
}

impl Display for IRTypedIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ident, self._type)
    }
}

impl Display for BlockStmt {
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

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "call %{}({})", self.name, self.args.to_string())
    }
}

impl Display for ArithOpExpr {
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

impl Display for IRStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            IRStmt::DeclaredFunction(func) => func.to_string(),
            IRStmt::Function(func) => func.to_string(),
            IRStmt::Variable(var) => var.to_string(),
            IRStmt::Constant(_const) => _const.to_string(),
            IRStmt::Label(label) => label.to_string(),
            IRStmt::Return(ret) => ret.to_string(),
            IRStmt::Break(br) => br.to_string(),
            IRStmt::Jump(jump) => jump.to_string(),
            IRStmt::Call(call) => call.to_string(),
            IRStmt::Expression(expr) => expr.to_string(),
        })
    }
}

impl Display for IRExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            IRExpr::Call(call) => call.to_string(),
            IRExpr::Literal(lit) => lit.to_string(),
            IRExpr::ArithOp(op) => op.to_string(),
            IRExpr::Ident(string) => string.into(),
        })
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "l{{{}}}",
            &match self {
                Literal::String(string) => format!("\"{}\"", string),
                Literal::Char(char) => char.to_string(),
                Literal::ShortFloat(_, val) => val.to_string(),
                Literal::LongFloat(_, val) => val.to_string(),
                Literal::Bool(val) => val.to_string(),
                Literal::Integer(_, val) => val.to_string(),
                Literal::Array(_, val) => format!("[{}]", val.to_string()),
                Literal::Vector(val) => format!("<{}>", val.to_string()),
            }
        )
    }
}
