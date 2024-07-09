//! Abstract Syntax Tree for the language

use std::collections::HashMap;

pub type FunctionTable<'ft> = HashMap<&'ft str, FunctionInfo<'ft>>;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo<'fi> {
    pub name: &'fi str,
    pub args: Vec<TypedIdent<'fi>>,
    pub ret_type: Type<'fi>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'ast> {
    Let(LetStatement<'ast>),
    Fn(FnStatement<'ast>),
    If(IfStatement<'ast>),
    Loop(LoopStatement<'ast>),
    Return(ReturnStatement<'ast>),
    
    Block(BlockStatement<'ast>),
    Expression(Expression<'ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'ast> {
    Call(CallExpression<'ast>),
    Infix(InfixOpExpr<'ast>),
    Literal(Literal<'ast>),
}

pub type Ident<'ast> = &'ast str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type<'ast> {
    Ident(Ident<'ast>),
    Array(&'ast Type<'ast>, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Reassign,
    Equals
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'ast> {
    Ident(Ident<'ast>),
    // TODO: Use an array for this?
    String(&'ast str),
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement<'ast> {
    pub condition: Expression<'ast>,
    pub block: BlockStatement<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnStatement<'ast> {
    pub name: Ident<'ast>,
    pub args: Vec<TypedIdent<'ast>>,
    pub ret_type: Type<'ast>,
    pub block: BlockStatement<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'ast> {
    pub name: TypedIdent<'ast>,
    pub val: Expression<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement<'ast> {
    pub stmts: Vec<Statement<'ast>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement<'ast> {
    pub condition: Option<Expression<'ast>>,
    pub block: BlockStatement<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'ast> {
    pub val: Expression<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression<'ast> {
    pub name: Ident<'ast>,
    pub args: Vec<Expression<'ast>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixOpExpr<'ast> {
    pub operator: Operator,
    pub sides: (&'ast Expression<'ast>, &'ast Expression<'ast>)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypedIdent<'ast> {
    pub _type: Type<'ast>,
    pub ident: Ident<'ast>,
}