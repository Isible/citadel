#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use bumpalo::Bump;
    use citadel_irparser::{IRLexer, IRParser};

    use crate::{api::Backend, x86::{TargetX86_64, X86Backend}};

    #[test]
    fn test_asm_compiler() {
        // COPY PASTA CODE due to arenas :>
        let path = "tests/main.chir";
        let file_content = fs::read(path).unwrap();
        let lexer = IRLexer::new(std::str::from_utf8(&file_content).unwrap());
        let parser_arena = Bump::new();
        let mut parser = IRParser::new(&lexer, &parser_arena);
        let ir_stream = parser.parse_program();

        let codegen_arena = Bump::new();
        let backend = X86Backend::new(TargetX86_64, &codegen_arena);
        let asm_code = backend.generate(ir_stream);
        println!("asm: {:#?}", asm_code);
        //utils::compiler_output(utils::format(asm_code.as_slice()), PathBuf::from("build/asm/out.asm"));
    }
}
