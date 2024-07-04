use crate::asm::{
    self,
    elements::{
        AsmElement, DataSize, Instruction, Literal, MemAddr, Opcode, Operand, Register,
        SizedLiteral,
    },
};

#[inline(always)]
pub(crate) fn gen_mov_ins(target: Operand, val: Operand) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Mov,
        args: vec![target, val],
    })
}

#[inline(always)]
pub(crate) fn gen_call(label_id: &str) -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Call,
        args: vec![Operand::Ident(label_id.to_string())],
    })
}

#[inline(always)]
pub(crate) fn gen_ret() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Ret,
        args: vec![],
    })
}

#[inline(always)]
pub(crate) fn gen_syscall() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Syscall,
        args: vec![],
    })
}

/// Returns the memory adress of the rbp register at pos
#[inline(always)]
pub(crate) fn get_stack_location(pos: i32) -> Operand {
    Operand::MemAddr(MemAddr::RegisterPos(Register::Rbp, pos))
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn destroy_stackframe() -> AsmElement {
    AsmElement::Instruction(Instruction {
        opcode: Opcode::Pop,
        args: vec![Operand::Register(Register::Rbp)],
    })
}

/// `size` is the size in bytes
#[inline(always)]
pub(crate) fn arg_regs_by_size(size: u8) -> [Register; 6] {
    match size {
        1 => asm::codegen::FUNCTION_ARG_REGISTERS_8,
        2 => asm::codegen::FUNCTION_ARG_REGISTERS_16,
        4 => asm::codegen::FUNCTION_ARG_REGISTERS_32,
        8 => asm::codegen::FUNCTION_ARG_REGISTERS_64,
        _ => panic!("Invalid size: {size}"),
    }
}

/// returns the size of the specified integer in bytes
#[inline(always)]
pub(crate) fn int_size(int: &str) -> u8 {
    match int {
        "i8" => 1,
        "i16" => 2,
        "i32" => 4,
        "i64" => 8,
        _ => unreachable!(),
    }
}

pub(crate) fn conv_str_to_bytes(string: &str) -> u64 {
    let mut res = 0;
    for (i, ch) in string.chars().enumerate() {
        res |= (ch as u64) << (i * 8);
    }
    res
}

pub(crate) fn split_string(input: &str, sub_string_len: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let end = (start + sub_string_len).min(input.len());
        result.push(&input[start..end]);
        start = end;
    }

    result
}

// size from word can be obtained by calling word.size()
#[inline(always)]
pub(crate) fn word_from_size(size: u8) -> DataSize {
    match size {
        1 => DataSize::Byte,
        2 => DataSize::Word,
        4 => DataSize::DWord,
        8 => DataSize::QWord,
        size => panic!("Size {size:?} is not valid for word. Valid sizes are: 1, 2, 4, 8 bytes"),
    }
}

#[inline(always)]
pub(crate) fn literal_to_sized_literal(literal: Literal) -> Option<SizedLiteral> {
    Some(SizedLiteral(
        literal,
        match literal {
            Literal::Int8(_) => DataSize::Byte,
            Literal::Int16(_) => DataSize::Word,
            Literal::Int32(_) => DataSize::DWord,
            Literal::Int64(_) => DataSize::QWord,
            Literal::Float32(_) | Literal::Float64(_) => return None,
        },
    ))
}
