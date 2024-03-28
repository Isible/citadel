//! This module contains data structures and ast-like representations
//! of assembly concepts. This is used to represent the assembly
//! the compiler outputs.

pub mod traits;

// TODO: Rewrite to support simple codegen

use strum::AsRefStr;

#[derive(Debug, Clone, PartialEq)]
pub enum AsmElement {
    Label(Label),
    Instruction(Instruction),
    Directive(Directive),
    Operand(Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub _type: DirectiveType,
    pub content: Vec<Declaration>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Global(String),
    DefineBytes(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub args: Vec<Operand>
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    Data,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub elements: Vec<AsmElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Ident(String),
    Register(Register),
    MemAddr(MemAddr),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemAddr {
    Register(Register),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Ident(String),
}

// TODO: Remove strum as a dependency
#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,

    Rdi,
    Rsi,
    Rbp,
    Rsp,
}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum Opcode {
    Mov,
    Syscall,

    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    XOr,
    Not,
    Cmp,

    Jmp,
    JE,
    JNe,
    JZ,
    JNz,

    Call,
    Ret,

    Push,
    Pop,

    Shl,
    Shr,

    Movsb,
    Movsw,
    Int,

    Fadd,
    Fsub,
    FMul,
    FDiv,

    FCmp,
    FAbs,

    Dec,
    Inc,
}