//! The AST (Abstract Syntax Tree) module for the Intermediary Representation (IR)
//! This has the nodes (statements, expressions) for generating an IR
//!
//! It is recommended to use the [IR-Generator](frontend::ir::ir_gen) module for generating the IR
//! but you can of course also generate the ir yourself

pub mod traits;
pub mod irgen;

#[derive(Debug, Clone, PartialEq)]
pub enum IRStmt {
    DeclaredFunction(DeclFuncStmt),
    Function(FuncStmt),
    Variable(VarStmt),
    Label(LabelStmt),

    Return(ReturnStmt),
    Break(BreakStmt),
    Jump(JumpStmt),
    Call(CallExpr),

    Expression(IRExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRExpr {
    Call(CallExpr),
    Literal(Literal),
    Ident(String),

    ArithOp(ArithOpExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Char(char),

    ShortFloat(u8, f32),
    LongFloat(u8, f64),
    
    /// Bool is an i1. 
    Bool(bool),

    Integer(u8, isize),

    Array(usize, Vec<IRExpr>),
    Vector(Vec<IRExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclFuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
    pub is_local: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
    pub block: BlockStmt,
    pub is_local: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: IRTypedIdent,
    pub val: IRExpr,
    pub is_local: bool,
    pub is_const: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelStmt {
    pub name: String,
    pub block: BlockStmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub ret_val: IRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JumpStmt {
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRTypedIdent {
    pub ident: String,
    pub _type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub stmts: Vec<IRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub name: String,
    pub args: Vec<IRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithOpExpr {
    pub op: Operator,
    pub values: (Box<IRExpr>, Box<IRExpr>)
}
