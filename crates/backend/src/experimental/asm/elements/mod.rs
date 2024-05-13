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

pub trait Size {
    fn size(&self) -> u8;
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

impl Size for Operand {
    fn size(&self) -> u8 {
        match self {
            Operand::Ident(_) => todo!(),
            Operand::Register(reg) => reg.size(),
            Operand::MemAddr(_) => 64,
            Operand::Literal(lit) => lit.size(),
            Operand::SizedLiteral(_, _) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSize {
    Byte,
    Word,
    DWord,
    QWord,
}

impl Size for DataSize {
    fn size(&self) -> u8 {
        match self {
            DataSize::Byte => 8,
            DataSize::Word => 16,
            DataSize::DWord => 32,
            DataSize::QWord => 64,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemAddr {
    Register(Register),
    RegisterPos(Register, i32),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    // FIXME: Although this is a u64 we only support 32 bit strings right now
    /// String, encoded using the little endian method
    String(u64),
}

impl Size for Literal {
    fn size(&self) -> u8 {
        match self {
            Literal::Int(_) => 32,
            Literal::Float(_) => 32,
            Literal::String(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // 16 bit
    Ax,
    Bx,
    Cx,
    Dx,

    Si,
    Di,
    Sp,
    Bp,

    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,

    // 8 bit
    Al,
    Bl,
    Cl,
    Dl,

    Sil,
    Dil,
    Spl,
    Bpl,

    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

impl Size for Register {
    fn size(&self) -> u8 {
        match self {
            Register::Rax
            | Register::Rbx
            | Register::Rcx
            | Register::Rdx
            | Register::Rsi
            | Register::Rdi
            | Register::Rsp
            | Register::Rbp
            | Register::R8
            | Register::R9
            | Register::R10
            | Register::R11
            | Register::R12
            | Register::R13
            | Register::R14
            | Register::R15 => 64,
            Register::Eax
            | Register::Ebx
            | Register::Ecx
            | Register::Edx
            | Register::Edi
            | Register::Esi
            | Register::Ebp
            | Register::Esp
            | Register::R8d
            | Register::R9d
            | Register::R10d
            | Register::R11d
            | Register::R12d
            | Register::R13d
            | Register::R14d
            | Register::R15d => 32,
            Register::Ax
            | Register::Bx
            | Register::Cx
            | Register::Dx
            | Register::Si
            | Register::Di
            | Register::Sp
            | Register::Bp
            | Register::R8w
            | Register::R9w
            | Register::R10w
            | Register::R11w
            | Register::R12w
            | Register::R13w
            | Register::R14w
            | Register::R15w => 16,
            Register::Al
            | Register::Bl
            | Register::Cl
            | Register::Dl
            | Register::Sil
            | Register::Dil
            | Register::Spl
            | Register::Bpl
            | Register::R8b
            | Register::R9b
            | Register::R10b
            | Register::R11b
            | Register::R12b
            | Register::R13b
            | Register::R14b
            | Register::R15b => 8,
        }
    }
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
                    util::gen_mov_ins(
                        Operand::Register(Register::Rax),
                        Operand::Literal(Literal::Int(1)),
                    ),
                    util::gen_mov_ins(
                        Operand::Register(Register::Rdi),
                        Operand::Literal(Literal::Int(1)),
                    ),
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
