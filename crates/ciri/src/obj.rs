use frontend::ir::{BlockStmt, IRTypedIdent, Literal};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Literal(Literal),
    Ret(Box<Object>),
    Br(String),
    Jmp(String),
    FuncObj(FuncObj),
    Label(LabelObj),
    // debuging
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncObj {
    pub(crate) args: Vec<IRTypedIdent>,
    pub(crate) block: BlockStmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelObj {
    pub(crate) block: BlockStmt,
}