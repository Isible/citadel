use citadel_frontend::ir::{self, IRExpr};

use crate::experimental::asm::elements::{AsmElement, Instruction, MemAddr, Opcode, Operand, Register};

pub(crate) fn gen_mov_ins(target: Operand, val: Operand) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Mov,
        args: vec![target, val],
    })
}

pub(crate) fn gen_call(label_id: &str) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Call,
        args: vec![Operand::Ident(label_id.to_string())],
    })
}

pub(crate) fn gen_ret() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Ret,
        args: vec![],
    })
}

pub(crate) fn gen_syscall() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Syscall,
        args: vec![],
    })
}

pub(crate) fn get_stack_location(pos: i32) -> Operand {
    Operand::MemAddr(MemAddr::RegisterPos(Register::Rbp, pos))
}

pub(crate) fn create_stackframe() -> (AsmElement, AsmElement) {
    (
        AsmElement::Instruction(Instruction {
            opcode: Opcode::Push,
            args: vec![Operand::Register(Register::Rbp)],
        }),
        gen_mov_ins(Operand::Register(Register::Rbp), Operand::Register(Register::Rsp)),
    )
}

pub(crate) fn destroy_stackframe() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Pop,
        args: vec![Operand::Register(Register::Rbp)],
    })
}

pub(super) fn string_from_lit(lit: &IRExpr) -> &String {
    match lit {
        IRExpr::Literal(ir::Literal::String(s)) => s,
        _ => panic!("Expected string literal"),
    }
}

pub(super) fn arg_regs_by_size(size: u8) -> [Register; 6] {
    match size {
        8 => super::FUNCTION_ARG_REGISTERS_8,
        16 => super::FUNCTION_ARG_REGISTERS_16,
        32 => super::FUNCTION_ARG_REGISTERS_32,
        64 => super::FUNCTION_ARG_REGISTERS_64,
        _ => panic!("Invalid size: {size}")
    }
}