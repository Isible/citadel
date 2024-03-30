use citadel_frontend::ir::{self, IRExpr};

use crate::experimental::asm::elements::{AsmElement, Instruction, Opcode, Operand, Register};

pub fn gen_mov_ins(reg: Register, val: Operand) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Mov,
        args: vec![Operand::Register(reg), val],
    })
}

pub fn gen_call(label_id: &str) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Call,
        args: vec![Operand::Ident(label_id.to_string())],
    })
}

pub fn gen_ret() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Ret,
        args: vec![],
    })
}

pub fn gen_syscall() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Syscall,
        args: vec![],
    })
}

pub fn string_from_lit(lit: &IRExpr) -> &String {
    match lit {
        IRExpr::Literal(ir::Literal::String(s)) => s,
        _ => panic!("Expected string literal"),
    }
}