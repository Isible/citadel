#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Fn(FnStatement),
    If(IfStatement),
    Loop(LoopStatement),

    Call(CallStatement),
    Block(BlockStatement)
}

#[derive(Debug)]
pub enum Expression {
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
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Ident,
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
pub struct CallStatement {
    pub name: Ident,
    pub args: Vec<TypedIdent>,
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