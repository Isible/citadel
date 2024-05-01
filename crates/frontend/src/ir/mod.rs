//! The AST (Abstract Syntax Tree) module for the Intermediary Representation (IR)
//! This has the nodes (statements, expressions) for generating an IR
//!
//! It is recommended to use the [IR-Generator](frontend::ir::ir_gen) module for generating the IR
//! but you can of course also generate the ir yourself

use std::ops::Deref;

pub mod traits;
pub mod irgen;

#[derive(Debug, Clone, PartialEq)]
pub enum IRStmt<'ir> {
    DeclaredFunction(DeclFuncStmt<'ir>),
    Function(FuncStmt<'ir>),
    Variable(VarStmt<'ir>),
    Label(LabelStmt<'ir>),

    Return(ReturnStmt<'ir>),
    Exit(ExitStmt<'ir>),
    Break(BreakStmt<'ir>),
    Jump(JumpStmt<'ir>),
    Call(CallExpr<'ir>),

    Struct(StructStmt<'ir>),
    Union(UnionStmt<'ir>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRExpr<'ir> {
    Call(CallExpr<'ir>),
    Literal(Literal<'ir>, Ident<'ir>),
    Ident(Ident<'ir>),

    ArithOp(ArithOpExpr<'ir>),

    StructInit(StructInitExpr<'ir>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'ir> {
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

    Array(usize, Vec<IRExpr<'ir>>),
    Vector(Vec<IRExpr<'ir>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident<'ir>(pub &'ir str);

impl<'ir> Deref for Ident<'ir> {
    type Target = &'ir str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclFuncStmt<'ir> {
    pub name: IRTypedIdent<'ir>,
    pub args: Vec<IRTypedIdent<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncStmt<'ir> {
    pub name: IRTypedIdent<'ir>,
    pub args: Vec<IRTypedIdent<'ir>>,
    pub block: BlockStmt<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt<'ir> {
    pub name: IRTypedIdent<'ir>,
    pub val: IRExpr<'ir>,
    pub is_const: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructStmt<'ir> {
    pub name: Ident<'ir>,
    pub fields: Vec<IRTypedIdent<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionStmt<'ir> {
    pub name: Ident<'ir>,
    pub variants: Vec<IRTypedIdent<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelStmt<'ir> {
    pub name: Ident<'ir>,
    pub block: BlockStmt<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'ir> {
    pub ret_val: IRExpr<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitStmt<'ir> {
    pub exit_code: IRExpr<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt<'ir> {
    pub label: Ident<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JumpStmt<'ir> {
    pub label: Ident<'ir>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IRTypedIdent<'ir> {
    pub ident: Ident<'ir>,
    pub _type: Ident<'ir>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt<'ir> {
    pub stmts: Vec<IRStmt<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr<'ir> {
    pub name: Ident<'ir>,
    pub args: Vec<IRExpr<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructInitExpr<'ir> {
    pub name: Ident<'ir>,
    pub values: Vec<IRExpr<'ir>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithOpExpr<'ir> {
    pub op: Operator,
    pub values: (Box<IRExpr<'ir>>, Box<IRExpr<'ir>>)
}
