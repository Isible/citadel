//! Trait implementations for asm elements, mainly the Display trait

use std::fmt::Display;

use crate::{experimental::asm::elements::DirectiveType, util::VecDisplay};

use crate::experimental::asm::elements::{
    AsmElement, Declaration, Directive, Instruction, Label, Literal, MemAddr, Opcode, Operand,
    Register,
};

use super::DataSize;

impl Display for AsmElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AsmElement::Label(label) => label.to_string(),
                AsmElement::Instruction(ins) => ins.to_string(),
                AsmElement::Directive(dir) => dir.to_string(),
                AsmElement::Operand(op) => op.to_string(),
                AsmElement::Declaration(decl) => decl.to_string(),
            }
        )
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Declaration::Global(ident) => format!("global {}", ident),
                Declaration::DefineBytes(ident, lit, terminator) =>
                    format!("{} db \"{}\", {:?}", ident, lit, terminator),
            }
        )
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.opcode,
            if !self.args.is_empty() {
                format!(" {}", self.args.to_string())
            } else {
                String::new()
            }
        )
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "section .{}",
            match self._type {
                DirectiveType::Data => "data",
                DirectiveType::Rodata => "rodata",
                DirectiveType::Text => "text",
            },
        )
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operand::Register(regis) => regis.to_string(),
                Operand::MemAddr(addr) => addr.to_string(),
                Operand::Literal(lit) => lit.to_string(),
                Operand::SizedLiteral(lit, data_size) => format!("{} {}", data_size, lit),
                Operand::Ident(ident) => ident.to_string(),
            }
        )
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataSize::Byte => "byte",
                DataSize::Word => "word",
                DataSize::DWord => "dword",
                DataSize::QWord => "qword",
            }
        )
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::Rax => "rax",
                Register::Rbx => "rbx",
                Register::Rcx => "rcx",
                Register::Rdx => "rdx",
                Register::Rdi => "rdi",
                Register::Rsi => "rsi",
                Register::Rbp => "rbp",
                Register::Rsp => "rsp",
                Register::R8 => "r8",
                Register::R9 => "r9",
                Register::R10 => "r10",
                Register::R11 => "r11",
                Register::R12 => "r12",
                Register::R13 => "r13",
                Register::R14 => "r14",
                Register::R15 => "r15",

                Register::Eax => "eax",
                Register::Ebx => "ebx",
                Register::Ecx => "ecx",
                Register::Edx => "edx",
                Register::Edi => "edi",
                Register::Esi => "esi",
                Register::Ebp => "ebp",
                Register::Esp => "esp",
                Register::R8d => "r8d",
                Register::R9d => "r9d",
                Register::R10d => "r10d",
                Register::R11d => "r11d",
                Register::R12d => "r12d",
                Register::R13d => "r13d",
                Register::R14d => "r14d",
                Register::R15d => "r15d",

                Register::Ax => "ax",
                Register::Bx => "bx",
                Register::Cx => "cx",
                Register::Dx => "dx",
                Register::Di => "di",
                Register::Si => "si",
                Register::Bp => "bp",
                Register::Sp => "sp",
                Register::R8w => "r8w",
                Register::R9w => "r9w",
                Register::R10w => "r10w",
                Register::R11w => "r11w",
                Register::R12w => "r12w",
                Register::R13w => "r13w",
                Register::R14w => "r14w",
                Register::R15w => "r15w",

                Register::Al => "al",
                Register::Bl => "bl",
                Register::Cl => "cl",
                Register::Dl => "dl",
                Register::Sil => "sil",
                Register::Dil => "dil",
                Register::Spl => "spl",
                Register::Bpl => "bpl",
                Register::R8b => "r8b",
                Register::R9b => "r9b",
                Register::R10b => "r10b",
                Register::R11b => "r11b",
                Register::R12b => "r12b",
                Register::R13b => "r13b",
                Register::R14b => "r14b",
                Register::R15b => "r15b",
            }
        )
    }
}

impl Display for MemAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            match self {
                MemAddr::Register(reg) => reg.to_string(),
                MemAddr::Literal(lit) => lit.to_string(),
                MemAddr::RegisterPos(reg, pos) => format!("{}{}", reg, pos),
            }
        )
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Literal::Int(int) => int.to_string(),
                Literal::Float(float) => float.to_string(),
                Literal::String(string) => string.to_string(),
            }
        )
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Opcode::Mov => "mov",
                Opcode::Syscall => "syscall",
                Opcode::Add => "add",
                Opcode::Sub => "sub",
                Opcode::Mul => "mul",
                Opcode::Div => todo!(),
                Opcode::And => todo!(),
                Opcode::Or => todo!(),
                Opcode::XOr => todo!(),
                Opcode::Not => todo!(),
                Opcode::Cmp => todo!(),
                Opcode::Jmp => todo!(),
                Opcode::JE => todo!(),
                Opcode::JNe => todo!(),
                Opcode::JZ => todo!(),
                Opcode::JNz => todo!(),
                Opcode::Call => "call",
                Opcode::Ret => "ret",
                Opcode::Push => "push",
                Opcode::Pop => "pop",
                Opcode::Shl => todo!(),
                Opcode::Shr => todo!(),
                Opcode::Movsb => todo!(),
                Opcode::Movsw => todo!(),
                Opcode::Int => todo!(),
                Opcode::Fadd => todo!(),
                Opcode::Fsub => todo!(),
                Opcode::FMul => todo!(),
                Opcode::FDiv => todo!(),
                Opcode::FCmp => todo!(),
                Opcode::FAbs => todo!(),
                Opcode::Dec => todo!(),
                Opcode::Inc => todo!(),
            }
        )
    }
}
