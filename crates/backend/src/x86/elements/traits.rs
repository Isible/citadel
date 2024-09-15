//! Trait implementations for asm elements, mainly the Display trait

use super::{ByteSize, DataSize, Register};

impl ByteSize for Register {
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

impl ByteSize for DataSize {
    fn size(&self) -> u8 {
        match self {
            DataSize::Byte => 1,
            DataSize::Word => 2,
            DataSize::DWord => 4,
            DataSize::QWord => 8,
        }
    }
}
