/* The AST (Abstract Syntax Tree) for the Intermediary Representation
 * This has the nodes (statements, expressions) for the ir
 */

pub mod traits;

#[derive(Debug)]
pub enum IRStmt {
    DeclaredFunction(DeclFuncStmt),
    Function(FuncStmt),
    Variable(VarStmt),
    Constant(ConstStmt),
    Label(LabelStmt),

    Return(ReturnStmt),
    Break(BreakStmt),
    Jump(JumpStmt),
    Call(CallExpr),

    Expression(IRExpr),
}

#[derive(Debug)]
pub enum IRExpr {
    Call(CallExpr),
    Literal(Literal),
    Ident(String),

    ArithOp(ArithOpExpr),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct DeclFuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct FuncStmt {
    pub name: IRTypedIdent,
    pub args: Vec<IRTypedIdent>,
    pub block: BlockStmt,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct VarStmt {
    pub name: IRTypedIdent,
    pub val: IRExpr,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct ConstStmt {
    pub name: IRTypedIdent,
    pub val: IRExpr,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct LabelStmt {
    pub name: String,
    pub block: BlockStmt,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub ret_val: IRExpr,
}

#[derive(Debug)]
pub struct BreakStmt {
    pub label: String,
}

#[derive(Debug)]
pub struct JumpStmt {
    pub label: String,
}

#[derive(Debug)]
pub struct IRTypedIdent {
    pub ident: String,
    pub _type: String,
}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<IRStmt>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub name: String,
    pub args: Vec<IRExpr>,
}

#[derive(Debug)]
pub struct ArithOpExpr {
    pub op: Operator,
    pub values: (Box<IRExpr>, Box<IRExpr>)
}
