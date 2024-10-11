//! The AST (Abstract Syntax Tree) module for the Intermediary Representation (IR)
//! This has the nodes (statements, expressions) for generating an IR
//!
//! It is recommended to use the [IR-Generator](frontend::ir::ir_gen) module for generating the IR
//! but you can of course also generate the ir yourself (then you have to build your own typetable however)

pub mod traits;
pub mod irgen;

pub use self::irgen::TypeTable;

pub const INT8_T: &str = "i8";
pub const INT16_T: &str = "i16";
pub const INT32_T: &str = "i32";
pub const INT64_T: &str = "i64";
pub const FLOAT32_T: &str = "f32";
pub const FLOAT64_T: &str = "f64";

#[derive(Debug, Clone, PartialEq)]
pub enum IRStmt<'ir> {
    Entry(BlockStmt<'ir>),

    DeclaredFunction(DeclFuncStmt<'ir>),
    Function(FuncStmt<'ir>),
    Variable(VarStmt<'ir>),
    Label(LabelStmt<'ir>),

    Return(ReturnStmt<'ir>),
    Exit(ExitStmt<'ir>),
    Jump(JumpStmt<'ir>),
    Call(CallExpr<'ir>),

    Struct(StructStmt<'ir>),
    Union(UnionStmt<'ir>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRExpr<'ir> {
    Call(CallExpr<'ir>),
    Literal(Literal<'ir>, Type<'ir>),
    Ident(Ident<'ir>),

    BinOp(BinOpExpr<'ir>),

    StructInit(StructInitExpr<'ir>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type<'ir> {
    Ident(Ident<'ir>),
    Array(&'ir Type<'ir>, u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'ir> {
    String(&'ir str),
    Char(u8),

    Float32(f32),
    Float64(f64),
    
    /// Bool is an i1. 
    Bool(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),

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

pub type Ident<'ir> = &'ir str;


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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelStmt<'ir> {
    pub name: Ident<'ir>,
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
pub struct JumpStmt<'ir> {
    pub label: Ident<'ir>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IRTypedIdent<'ir> {
    pub ident: Ident<'ir>,
    pub _type: Type<'ir>,
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
pub struct BinOpExpr<'ir> {
    pub op: Operator,
    pub values: (Box<IRExpr<'ir>>, Box<IRExpr<'ir>>)
}
