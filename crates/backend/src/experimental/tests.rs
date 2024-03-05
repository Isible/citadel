#[cfg(test)]
mod tests {
    use crate::experimental::util;


    #[test]
    fn test_asm_codegen() {
        /*
        let asm_code = AsmElement::Label(Label {
            name: "start".into(),
            block: Block {
                elements: vec![Instruction {
                    _type: InstructionType::Mov,
                    args: vec![
                        Operand::Register(Register::Rax),
                        Operand::Literal(Literal::Int(10)),
                    ],
                }],
            },
        });
        println!("{}", asm_code);
        util::compiler_output(vec![asm_code], "tests/out/out.asm");
        */
        let program = util::compile_program(vec![]);
        println!("Program: {:#?}", program);
    }
}
