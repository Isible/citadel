#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Fn(FnStatement),
    If(IfStatement),
    Loop(LoopStatement),

    Call(CallExpression),
    Block(BlockStatement)
}

#[derive(Debug)]
pub enum Expression {
    Call(CallExpression),
    ArithmeticOperation(ArithmeticOperationExpr),
    Literal(Literal),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Multiply,
    Reassign,
    Equals
}

#[derive(Debug)]
pub enum Literal {
    Ident(String),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub struct FnStatement {
    pub name: Ident,
    pub args: Vec<TypedIdent>,
    pub ret_type: Ident,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: TypedIdent,
    pub val: Expression,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>
}

#[derive(Debug)]
pub struct LoopStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct CallExpression {
    pub name: Ident,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub struct ArithmeticOperationExpr {
    pub operator: Operator,
    pub sides: (Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub struct TypedIdent {
    pub _type: Ident,
    pub ident: Ident,
}