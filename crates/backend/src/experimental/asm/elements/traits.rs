//! Trait implementations for asm elements, mainly the Display trait

use std::fmt::Display;

use crate::{experimental::asm::elements::DirectiveType, util::VecDisplay};

use crate::experimental::asm::elements::{
    AsmElement, Block, Declaration, Directive, Instruction, InstructionType, Label, Literal,
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
                Declaration::DefineBytes => todo!(),
            }
        )
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.name, self.block)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self._type, self.args.to_string())
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "section .{}\n{}",
            match self._type {
                DirectiveType::Data => "data",
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
                Literal::Ident(ref ident) => ident.into(),
            }
        )
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf: Vec<String> = self.elements.iter().map(|elem| elem.to_string()).collect();
        write!(f, "{}", buf.join("\n"))
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InstructionType::Mov => "mov",
                InstructionType::Syscall => "syscall",
                InstructionType::Add => "add",
                InstructionType::Sub => "sub",
                InstructionType::Mul => "mul",
                InstructionType::Div => todo!(),
                InstructionType::And => todo!(),
                InstructionType::Or => todo!(),
                InstructionType::XOr => todo!(),
                InstructionType::Not => todo!(),
                InstructionType::Cmp => todo!(),
                InstructionType::Jmp => todo!(),
                InstructionType::JE => todo!(),
                InstructionType::JNe => todo!(),
                InstructionType::JZ => todo!(),
                InstructionType::JNz => todo!(),
                InstructionType::Call => todo!(),
                InstructionType::Ret => todo!(),
                InstructionType::Push => todo!(),
                InstructionType::Pop => todo!(),
                InstructionType::Shl => todo!(),
                InstructionType::Shr => todo!(),
                InstructionType::Movsb => todo!(),
                InstructionType::Movsw => todo!(),
                InstructionType::Int => todo!(),
                InstructionType::Fadd => todo!(),
                InstructionType::Fsub => todo!(),
                InstructionType::FMul => todo!(),
                InstructionType::FDiv => todo!(),
                InstructionType::FCmp => todo!(),
                InstructionType::FAbs => todo!(),
                InstructionType::Dec => todo!(),
                InstructionType::Inc => todo!(),
            }
        )
    }
}
