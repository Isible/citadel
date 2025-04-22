#[cfg(test)]
mod tests {
    use std::fs;

    use bumpalo::Bump;
    use citadel_irparser::{IRLexer, IRParser};
    use citadel_middleend::{lir::optimization::ClirOptimization, optimize};

    use crate::{api::Backend, x86::{machine::lir_to_machine, MachineStream, TargetX86_32, TargetX86_64, X86Backend}};

    #[test]
    fn test_asm_compiler() {
        // COPY PASTA CODE due to arenas :>
        let path = "tests/main.chir";
        let file_content = fs::read(path).unwrap();
        // HIR
        let lexer = IRLexer::new(std::str::from_utf8(&file_content).unwrap());
        let parser_arena = Bump::new();
        let mut parser = IRParser::new(&lexer, &parser_arena);
        let hir_stream = parser.parse_program();
        // LIR
        let codegen_arena = Bump::new();
        let lir_stream = optimize!(hir_stream, ClirOptimization::new(&codegen_arena));

        let backend = X86Backend::new(TargetX86_64, &codegen_arena);
        backend.generate(lir_stream);
        //println!("asm: {:#?}", asm_code);
        //let mut gen = MachineGenerator::new(TargetX86_64);
        //gen.generate(asm_code);
        //utils::compiler_output(utils::format(asm_code.as_slice()), PathBuf::from("build/asm/out.asm"));
    }
}
