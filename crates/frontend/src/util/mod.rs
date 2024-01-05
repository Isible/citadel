use crate::ast::{IRExpr, IRTypedIdent};

use self::vec_display::VecDisplay;

pub mod vec_display;

impl VecDisplay for Vec<IRExpr> {
    fn to_string(&self) -> String {
        let mut exprs = Vec::new();
        for expr in self {
            exprs.push(expr.to_string());
            exprs.push(",".into());
        }
        exprs.pop();
        exprs.join("")
    }
}

impl VecDisplay for Vec<IRTypedIdent> {
    fn to_string(&self) -> String {
        let mut idents = Vec::new();
        for ident in self {
            idents.push(ident.to_string());
        }
        idents.join("")
    }
}