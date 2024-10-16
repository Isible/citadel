//! This module contains data structures and ast-like representations
//! of assembly concepts. This is used to represent the assembly
//! the compiler outputs.

pub mod traits;

#[derive(Debug, Clone, Copy)]
pub enum Instruction<'ins> {
    MovR2R {
        val: Register,
        dest: Register
    },
    MovI2R {
        val: i64,
        dest: Register,
    },
    MovM2R {
        val: (),
        dest: Register,
    },
    MovR2M {
        val: Register,
        dest: (),
    },
    Call {
        func: &'ins str,
    },
    Syscall,
}

impl<'ins> Instruction<'ins> {
    pub fn opcode(&self) -> &[u8] {
        match self {
            Instruction::MovR2R { val, dest } => todo!(),
            Instruction::MovI2R { val: _, dest } => match dest {
                Register::Rax => &[0xb8],
                Register::Rdi => &[0xbf],
                _ => todo!(),
            },
            Instruction::MovM2R { val, dest } => todo!(),
            Instruction::MovR2M { val, dest } => todo!(),
            Instruction::Call { func: _ } => &[0xe8],
            Instruction::Syscall => &[0x0f, 0x05],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
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

impl Register {
    pub fn code(&self) -> u8 {
        match self {
            Self::Rax => 0x0,
            Self::Rcx => 0x1,
            Self::Rdx => 0x2,
            Self::Rbx => 0x3,
            Self::Rsp => 0x4,
            Self::Rbp => 0x5,
            Self::Rsi => 0x6,
            Self::Rdi => 0x7,
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub enum DataValue {
    Str(&'static str),
}

pub trait ByteSize {
    /// Returns size in bytes
    fn size(&self) -> u8;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSize {
    Byte,
    Word,
    DWord,
    QWord,
}
