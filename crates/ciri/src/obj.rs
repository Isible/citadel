//! Objects for the interpreter. This is the most minimalistic
//! representation of a AST node possible.

use citadel_frontend::ir::{BlockStmt, IRTypedIdent, Literal};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object<'o> {
    Value(Literal<'o>),
    Ret(&'o Box<Object<'o>>),
    Br(&'o str),
    Jmp(&'o str),
    FuncObj(FuncObj<'o>),
    Label(LabelObj<'o>),
    // debuging
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncObj<'o> {
    pub(crate) args: Vec<IRTypedIdent<'o>>,
    pub(crate) block: BlockStmt<'o>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelObj<'o> {
    pub(crate) block: BlockStmt<'o>,
}