#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Fn(FnStatement),
    If(IfStatement),
    Loop(LoopStatement),

    Call(CallExpression),
    Block(BlockStatement)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Call(CallExpression),
    ArithmeticOperation(ArithmeticOperationExpr),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Multiply,
    Reassign,
    Equals
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Ident(String),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq)]
pub struct FnStatement {
    pub name: Ident,
    pub args: Vec<TypedIdent>,
    pub ret_type: Ident,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: TypedIdent,
    pub val: Expression,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>
}

#[derive(Debug, PartialEq)]
pub struct LoopStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub name: Ident,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ArithmeticOperationExpr {
    pub operator: Operator,
    pub sides: (Box<Expression>, Box<Expression>)
}

#[derive(Debug, PartialEq)]
pub struct TypedIdent {
    pub _type: Ident,
    pub ident: Ident,
}