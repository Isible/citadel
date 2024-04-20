//! This module contains data structures and ast-like representations
//! of assembly concepts. This is used to represent the assembly
//! the compiler outputs.

pub mod traits;

// TODO: Rewrite to support simple codegen

use crate::experimental::asm::codegen::util;

#[derive(Debug, Clone, PartialEq)]
pub enum AsmElement {
    Label(Label),
    Instruction(Instruction),
    Directive(Directive),
    Operand(Operand),
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub _type: DirectiveType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Global(String),
    DefineBytes(String, String, u8),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub args: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    Data,
    Rodata,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Ident(String),
    Register(Register),
    MemAddr(MemAddr),
    Literal(Literal),
    SizedLiteral(Literal, DataSize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSize {
    Byte,
    Word,
    DWord,
    QWord,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemAddr {
    Register(Register),
    RegisterPos(Register, i32),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    // 64 bit
    Rax,
    Rbx,
    Rcx,
    Rdx,

    Rsi,
    Rdi,
    Rsp,
    Rbp,

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    // 32 bit
    Eax,
    Ebx,
    Ecx,
    Edx,

    Edi,
    Esi,
    Ebp,
    Esp,

    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StdFunction {
    Print,
}

impl StdFunction {
    pub fn generate(&self) -> Vec<AsmElement> {
        match self {
            Self::Print => {
                vec![
                    AsmElement::Label(Label {
                        name: self.name().to_string(),
                    }),
                    util::gen_mov_ins(Operand::Register(Register::Rax), Operand::Literal(Literal::Int(1))),
                    util::gen_mov_ins(Operand::Register(Register::Rdi), Operand::Literal(Literal::Int(1))),
                    util::gen_syscall(),
                    util::gen_ret(),
                ]
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Print => "print",
        }
    }
}
