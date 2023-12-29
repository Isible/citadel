/* The AST (Abstract Syntax Tree) for the Intermediary Representation
 * This has the nodes (statements, expressions) for the ir
 */

pub mod traits;

#[derive(Debug)]
pub enum Statement {
    AbstractFunction(AbstFuncStmt),
    Function(FuncStmt),
    Variable(VarStmt),
    Constant(ConstStmt),
    Label(LabelStmt),

    Return(ReturnStmt),
    Break(BreakStmt),
    Goto(GotoStmt),

    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Call(CallExpr),
    Literal(Literal),
    Ident(String),

    Add(AddExpr),
    Sub(SubExpr),
    Multiply(MulExpr),
    Div(DivExpr),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Char(char),

    ShortFloat(u8, f32),
    LongFloat(u8, f64),
    
    /// Bool is an i1. 
    Bool(u8, bool),

    Integer(u8, isize),

    Array(usize, Vec<Expression>),
    Vector(usize, Vec<Expression>),
}

#[derive(Debug)]
pub struct AbstFuncStmt {
    pub name: TypedIdent,
    pub args: Vec<TypedIdent>,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct FuncStmt {
    pub name: TypedIdent,
    pub args: Vec<TypedIdent>,
    pub block: BlockStmt,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct VarStmt {
    pub name: TypedIdent,
    pub val: Expression,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct ConstStmt {
    pub name: TypedIdent,
    pub val: Expression,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct LabelStmt {
    pub name: String,
    pub block: BlockStmt,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub ret_val: Expression,
}

#[derive(Debug)]
pub struct BreakStmt {
    pub label: String,
}

#[derive(Debug)]
pub struct GotoStmt {
    pub label: String,
}

#[derive(Debug)]
pub struct TypedIdent {
    pub ident: String,
    pub _type: String,
}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub struct AddExpr {
    values: (Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub struct SubExpr {
    values: (Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub struct MulExpr {
    values: (Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub struct DivExpr {
    values: (Box<Expression>, Box<Expression>)
}