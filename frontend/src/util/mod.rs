use crate::ast::{Expression, TypedIdent};

use self::vec_display::VecDisplay;

pub mod vec_display;

impl VecDisplay for Vec<Expression> {
    fn to_string(&self) -> String {
        let mut exprs = Vec::new();
        for expr in self {
            exprs.push(expr.to_string());
        }
        exprs.join("")
    }
}

impl VecDisplay for Vec<TypedIdent> {
    fn to_string(&self) -> String {
        let mut idents = Vec::new();
        for ident in self {
            idents.push(ident.to_string());
        }
        idents.join("")
    }
}