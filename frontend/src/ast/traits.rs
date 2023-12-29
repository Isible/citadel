use std::fmt::Display;
use crate::util::vec_display::VecDisplay;

use super::*;

impl Display for AbstFuncStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "abst @{}({}) {} {}", self.name.ident, self.args.to_string(), if self.is_local {
            "lcl"
        } else {
            "pub"
        }, self.name._type)
    }
}

impl Display for FuncStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "abst @{}({}) {} {} {{\n{}\n}}", self.name.ident, self.args.to_string(), if self.is_local {
            "lcl"
        } else {
            "pub"
        }, self.name._type, self.block)
    }
}

impl Display for VarStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{} {} {} = {}", self.name, if self.is_local {
            "lcl"
        } else {
            "pub"
        }, self.name._type, self.val)
    }
}

impl Display for ConstStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${} {} {} = {}", self.name, if self.is_local {
            "lcl"
        } else {
            "pub"
        }, self.name._type, self.val)
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
        write!(f, "break {}", self.label)
    }
}

impl Display for GotoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "goto {}", self.label)
    }
}

impl Display for TypedIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ident, self._type)
    }
}

impl Display for BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmts = Vec::new();
        for stmt in &self.stmts {
            stmts.push(stmt.to_string());
            stmts.push("\n".into());
        }
        stmts.pop();
        write!(f, "{}", stmts.join(""))
    }
}

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "call {}({})", self.name, self.args.to_string())
    }
}

impl Display for AddExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add {}, {}", self.values.0, self.values.1)
    }
}

impl Display for SubExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sub {}, {}", self.values.0, self.values.1)
    }
}

impl Display for MulExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mul {}, {}", self.values.0, self.values.1)
    }
}

impl Display for DivExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "div {}, {}", self.values.0, self.values.1)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Statement::AbstractFunction(func) => func.to_string(),
            Statement::Function(func) => func.to_string(),
            Statement::Variable(var) => var.to_string(),
            Statement::Constant(_const) => _const.to_string(),
            Statement::Label(label) => label.to_string(),
            Statement::Return(ret) => ret.to_string(),
            Statement::Break(br) => br.to_string(),
            Statement::Goto(goto) => goto.to_string(),
            Statement::Expression(expr) => expr.to_string(),
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Expression::Call(call) => call.to_string(),
            Expression::Literal(lit) => lit.to_string(),
            Expression::Add(add) => add.to_string(),
            Expression::Sub(sub) => sub.to_string(),
            Expression::Multiply(mul) => mul.to_string(),
            Expression::Div(div) => div.to_string(),
        })
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Literal::String(string) => format!("\"{}\"", string),
            Literal::Char(char) => char.to_string(),
            Literal::ShortFloat(_, val) => val.to_string(),
            Literal::LongFloat(_, val) => val.to_string(),
            Literal::Bool(_, val) => val.to_string(),
            Literal::Integer(_, val) => val.to_string(),
            Literal::Array(_, val) => format!("[{}]", val.to_string()),
            Literal::Vector(_, val) => format!("<{}>", val.to_string()),
        })
    }
}
