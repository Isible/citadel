use std::fmt::Display;

use super::elements::{AsmElement, Block, Directive, Instruction, InstructionType, Label, Operand};

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

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.name, self.block)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self._type, self.args)
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
                InstructionType::Div => "div",
                InstructionType::And => todo!(),
                InstructionType::Or => todo!(),
                InstructionType::XOr => todo!(),
                InstructionType::Not => todo!(),
                InstructionType::Cmp => "cmp",
                InstructionType::Jmp => "jmp",
                InstructionType::JE => "je",
                InstructionType::JNe => "jne",
                InstructionType::JZ => "jz",
                InstructionType::JNz => "jnz",
                InstructionType::Call => "call",
                InstructionType::Ret => "ret",
                InstructionType::Push => "push",
                InstructionType::Pop => "pop",
                InstructionType::Shl => "shl",
                InstructionType::Shr => "shr",
                InstructionType::Movsb => todo!(),
                InstructionType::Movsw => todo!(),
                InstructionType::Int => "int",
                InstructionType::Fadd => "fadd",
                InstructionType::Fsub => "fsub",
                InstructionType::FMul => "fmul",
                InstructionType::FDiv => "fdiv",
                InstructionType::FCmp => "fcmp",
                InstructionType::FAbs => "fabs",
                InstructionType::Dec => "dec",
                InstructionType::Inc => "inc",
            }
        )
    }
}
