use std::fmt::Debug;

use crate::hir::{IRExpr, IRTypedIdent};

pub mod errors;

#[derive(Debug, Clone, Copy)]
pub enum CompositeDataType {
    Struct,
    Union,
}

pub(crate) trait VecDisplay: Debug {
    fn to_string(&self) -> String;
}

impl VecDisplay for Vec<IRExpr<'_>> {
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

impl VecDisplay for Vec<IRTypedIdent<'_>> {
    fn to_string(&self) -> String {
        let mut idents = Vec::new();
        for ident in self {
            idents.push(format!("${} {},", ident.ident, ident._type));
        }
        let mut idents = idents.join("");
        idents.pop();
        idents
    }
}
