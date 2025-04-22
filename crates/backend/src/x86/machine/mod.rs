use std::{io, ops::ControlFlow};

use byteorder::{ByteOrder, WriteBytesExt};
use citadel_middleend::lir::{self, ByteSize};

use crate::api::Target;

use super::{TargetX86_32, TargetX86_64};

mod traits;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction<'ins> {
    MovR2R { val: Register, dest: Register },
    MovI2R { val: Immediate, dest: Register },
    MovM2R { val: (), dest: Register },
    MovR2M { val: Register, dest: () },
    Call { func: &'ins str },
    Ret,
    Syscall,
}

impl<'ins> Instruction<'ins> {
    pub fn opcode<T: Target>(&self, target: T) -> &[u8] {
        match self {
            Instruction::MovR2R { val, dest } => todo!(),
            Instruction::MovI2R { val, dest } => match dest {
                Register::Eax => &[0xb8],
                Register::Ebx => &[0xbb],
                Register::Rax => match val {
                    Immediate::Int32(_) => &[0xb8],
                    Immediate::Int64(_) => &[0x48, 0xb8],
                },
                Register::Rdi => &[0x48, 0xbf],
                Register::Rbx => match val {
                    Immediate::Int32(_) => &[0xbb],
                    Immediate::Int64(_) => &[0x48, 0xbb],
                },
                _ => todo!(),
            },
            Instruction::MovM2R { val, dest } => todo!(),
            Instruction::MovR2M { val, dest } => todo!(),
            Instruction::Call { .. } => &[0xe8],
            Instruction::Ret => &[0xc3],
            Instruction::Syscall => {
                if target.name() == TargetX86_32.name() {
                    &[0xcd, 0x80]
                } else if target.name() == TargetX86_64.name() {
                    &[0x0f, 0x05]
                } else {
                    todo!()
                }
            }
        }
    }
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

    Esi,
    Edi,
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

pub const STANARD_BYTE_SIZE: [u8; 4] = [8, 16, 32, 64];

impl Register {
    pub fn by_size(byte_size: u8) -> Option<[Self; 16]> {
        match byte_size {
            8 => Some([
                Register::Al,
                Register::Bl,
                Register::Cl,
                Register::Dl,
                Register::Sil,
                Register::Dil,
                Register::Spl,
                Register::Bpl,
                Register::R8b,
                Register::R9b,
                Register::R10b,
                Register::R11b,
                Register::R12b,
                Register::R13b,
                Register::R14b,
                Register::R15b,
            ]),
            16 => Some([
                Register::Ax,
                Register::Bx,
                Register::Cx,
                Register::Dx,
                Register::Si,
                Register::Di,
                Register::Sp,
                Register::Bp,
                Register::R8b,
                Register::R9b,
                Register::R10w,
                Register::R11w,
                Register::R12w,
                Register::R13w,
                Register::R14w,
                Register::R15w,
            ]),
            32 => Some([
                Register::Eax,
                Register::Ebx,
                Register::Ecx,
                Register::Edx,
                Register::Esi,
                Register::Edi,
                Register::Esp,
                Register::Ebp,
                Register::R8d,
                Register::R9d,
                Register::R10d,
                Register::R11d,
                Register::R12d,
                Register::R13d,
                Register::R14d,
                Register::R15d,
            ]),
            64 => Some([
                Register::Rax,
                Register::Rbx,
                Register::Rcx,
                Register::Rdx,
                Register::Rsi,
                Register::Rdi,
                Register::Rsp,
                Register::Rbp,
                Register::R8,
                Register::R9,
                Register::R10,
                Register::R11,
                Register::R12,
                Register::R13,
                Register::R14,
                Register::R15,
            ]),
            _ => None,
        }
    }

    #[deprecated]
    /*pub*/
    fn code(&self) -> u8 {
        match self {
            Self::Rax => 0x0,
            Self::Rcx => 0x1,
            Self::Rdx => 0x2,
            Self::Rbx => 0x3,
            Self::Rsp => 0x4,
            Self::Rbp => 0x5,
            Self::Rsi => 0x6,
            Self::Rdi => 0x7,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Immediate {
    Int32(i32),
    Int64(i64),
}

impl Immediate {
    pub fn write<T: ByteOrder>(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self {
            Immediate::Int32(i) => buf.write_i32::<T>(*i),
            Immediate::Int64(i) => buf.write_i64::<T>(*i),
        }
    }
}

enum Value {
    Immediate(lir::Immediate),
    Memory(()),
    Register(lir::Register),
}

struct RegisterContext<T: Target> {
    target: T,
    val: Option<Value>,
}

pub fn lir_to_machine<T: Target>(
    target: T,
    instructions: Vec<lir::Instruction>,
) -> Vec<Instruction> {
    let mut machine_instructions = vec![];
    for ins in instructions {
        let machine_ins = match ins_to_machine_ins(target, ins) {
            Some(value) => value,
            None => continue,
        };
        machine_instructions.push(machine_ins);
    }
    machine_instructions
}

fn ins_to_machine_ins<T: Target>(target: T, ins: lir::Instruction<'_>) -> Option<Instruction<'_>> {
    Some(match ins {
        lir::Instruction::MovR2R { val, dest } => todo!(),
        lir::Instruction::MovI2R { val, dest } => {
            Instruction::MovI2R {
                val: immediate_to_machine_immediate(val),
                dest: {
                    let reg = reg_to_machine_reg(
                        dest,
                        RegisterContext {
                            target,
                            val: Some(Value::Immediate(val)),
                        },
                    );
                    if let Some(reg) = reg {
                        reg
                    } else {
                        eprintln!("Failed to convert register {dest} to machine register, affected: MovI2R");
                        return None;
                    }
                },
            }
        }
        lir::Instruction::MovM2R { val, dest } => todo!(),
        lir::Instruction::MovR2M { val, dest } => todo!(),
        lir::Instruction::Call { func } => Instruction::Call { func },
        lir::Instruction::Ret => Instruction::Ret,
        lir::Instruction::Syscall => Instruction::Syscall,
    })
}

fn immediate_to_machine_immediate(immediate: lir::Immediate) -> Immediate {
    match immediate {
        lir::Immediate::Int32(val) => Immediate::Int32(val),
        lir::Immediate::Int64(val) => Immediate::Int64(val),
    }
}

fn reg_to_machine_reg<T: Target>(
    register: lir::Register,
    ctx: RegisterContext<T>,
) -> Option<Register> {
    let machine_registers = match ctx.val {
        Some(Value::Immediate(val)) => Register::by_size(val.byte_size()),
        Some(Value::Register(_)) => todo!(),
        Some(Value::Memory(())) => todo!(),
        None => None,
    };
    machine_registers.map(|registers| registers[register.index()])
}
