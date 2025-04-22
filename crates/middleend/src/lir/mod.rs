//! This module contains data structures and ast-like representations
//! of assembly concepts. This is used to represent the assembly
//! the compiler outputs.

use std::{fmt::Display};

pub mod irgen;
pub mod optimization;

#[derive(Debug, Clone, Copy)]
pub enum Instruction<'ins> {
    MovR2R {
        val: Register,
        dest: Register,
    },
    MovI2R {
        val: Immediate,
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
    Ret,
    Syscall,
}

impl<'ins> Display for Instruction<'ins> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::MovR2R { val, dest } => write!(f, "mov {val}, {dest}"),
            Instruction::MovI2R { val, dest } => write!(f, "mov {val}, {dest}"),
            Instruction::MovM2R { val, dest } => todo!(),//write!(f, "mov {val}, {dest}"),
            Instruction::MovR2M { val, dest } => todo!(),//write!(f, "mov {val}, {dest}"),
            Instruction::Call { func } => write!(f, "call {func}"),
            Instruction::Ret => write!(f, "ret"),
            Instruction::Syscall => write!(f, "syscall"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Immediate {
    Int32(i32),
    Int64(i64),
}

impl ByteSize for Immediate {
    fn byte_size(&self) -> u8 {
        match self {
            Immediate::Int32(_) => 32,
            Immediate::Int64(_) => 64,
        }
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::Int32(i) => write!(f, "{i}"),
            Immediate::Int64(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,

    R4,
    R5,
    R6,
    R7,

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::R3 => "r3",
            Register::R4 => "r4",
            Register::R5 => "r5",
            Register::R6 => "r6",
            Register::R7 => "r7",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataValue {
    Str(&'static str),
}

pub trait ByteSize {
    /// Returns size in bytes
    fn byte_size(&self) -> u8;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSize {
    Byte,
    Word,
    DWord,
    QWord,
}
