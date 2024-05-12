use citadel_frontend::ir::{self, IRExpr};

use crate::experimental::asm::elements::{
    AsmElement, Instruction, MemAddr, Opcode, Operand, Register,
};

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

/// Returns the memory adress of the rbp register at pos
pub(crate) fn get_stack_location(pos: i32) -> Operand {
    Operand::MemAddr(MemAddr::RegisterPos(Register::Rbp, pos))
}

pub(crate) fn create_stackframe() -> (AsmElement, AsmElement) {
    (
        AsmElement::Instruction(Instruction {
            opcode: Opcode::Push,
            args: vec![Operand::Register(Register::Rbp)],
        }),
        gen_mov_ins(
            Operand::Register(Register::Rbp),
            Operand::Register(Register::Rsp),
        ),
    )
}

pub(crate) fn destroy_stackframe() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Pop,
        args: vec![Operand::Register(Register::Rbp)],
    })
}

pub(crate) fn string_from_lit<'s>(lit: &'s IRExpr<'s>) -> &'s String {
    match lit {
        IRExpr::Literal(ir::Literal::String(s), _) => s,
        _ => panic!("Expected string literal"),
    }
}

pub(crate) fn arg_regs_by_size(size: u8) -> [Register; 6] {
    match size {
        8 => super::FUNCTION_ARG_REGISTERS_8,
        16 => super::FUNCTION_ARG_REGISTERS_16,
        32 => super::FUNCTION_ARG_REGISTERS_32,
        64 => super::FUNCTION_ARG_REGISTERS_64,
        _ => panic!("Invalid size: {size}"),
    }
}

/// returns the size of the specified integer in bytes
pub(super) fn int_size(int: &str) -> u8 {
    match int {
        "i8" => 1,
        "i16" => 2,
        "i32" => 4,
        "i64" => 8,
        _ => unreachable!(),
    }
}

pub(super) fn conv_str_to_bytes(string: &str) -> u64 {
    let mut res = 0;
    for (i, ch) in string.chars().into_iter().enumerate() {
        res |= (ch as u64) << (i * 8);
    }
    res
}

pub(super) fn split_string(input: &str, sub_string_len: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let end = (start + sub_string_len).min(input.len());
        result.push(&input[start..end]);
        start = end;
    }

    result
}