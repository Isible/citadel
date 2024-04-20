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
    Ident(Ident),

    ArithOp(ArithOpExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    // TODO: Make this a byte
    Char(char),

    Float(f32),
    Double(f64),
    
    /// Bool is an i1. 
    Bool(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

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
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DeclFuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
    pub block: BlockStmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: IRTypedIdent,
    pub val: IRExpr,
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
