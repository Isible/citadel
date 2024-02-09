use std::fmt::Display;

use crate::util::VecDisplay;

use super::elements::{
    AsmElement, Block, Directive, Instruction, InstructionType, Label, Literal, MemAddr, Operand,
    Register,
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
        todo!()
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
        write!(f, "{}", stringify!(self).to_lowercase())
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
        write!(f, "{}", stringify!(self).to_lowercase())
    }
}
