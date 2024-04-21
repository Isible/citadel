//! Abstract Syntax Tree for the language

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Fn(FnStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
    
    Block(BlockStatement),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Call(CallExpression),
    Infix(InfixOpExpr),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Reassign,
    Equals
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Ident(String),
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnStatement {
    pub name: String,
    pub args: Vec<TypedIdent>,
    pub ret_type: String,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: TypedIdent,
    pub val: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub val: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixOpExpr {
    pub operator: Operator,
    pub sides: (Box<Expression>, Box<Expression>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIdent {
    pub _type: String,
    pub ident: String,
}