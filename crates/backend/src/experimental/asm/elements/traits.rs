//! Trait implementations for asm elements, mainly the Display trait

use std::fmt::Display;

use crate::{experimental::asm::elements::DirectiveType, util::VecDisplay};

use crate::experimental::asm::elements::{
    AsmElement, Declaration, Directive, Instruction, Opcode, Label, Literal,
    MemAddr, Operand, Register,
};

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
                Declaration::DefineBytes(ident, lit, terminator) => format!("{} db \"{}\", {:?}", ident, lit, terminator),
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
        write!(f, "{} {}", self.opcode, self.args.to_string())
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "section .{}\n{}",
            match self._type {
                DirectiveType::Data => "data",
                DirectiveType::Rodata => "rodata",
                DirectiveType::Text => "text",
            },
            self.content.to_string()
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
                Operand::Ident(ident) => ident.to_string(),
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
                MemAddr::Register(regis) => regis.to_string(),
                MemAddr::Literal(lit) => lit.to_string(),
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
                Literal::String(ref string) => string.into(),
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
                Opcode::Push => todo!(),
                Opcode::Pop => todo!(),
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
