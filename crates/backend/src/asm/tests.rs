#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use bumpalo::Bump;
    use citadel_irparser::{IRLexer, IRParser};

    use crate::{
        api::Backend,
        asm::{utils, AsmBackend, TargetX86_64},
    };

    #[test]
    fn test_asm_compiler() {
        // COPY PASTA CODE due to arenas :>
        let path = "tests/main.chir";
        let file_content = fs::read(path).unwrap();
        let lexer = IRLexer::new(std::str::from_utf8(&file_content).unwrap());
        let arena = Bump::new();
        let mut parser = IRParser::new(&lexer, &arena);
        let ir_stream = parser.parse_program();

        dbg!(&ir_stream);
        let backend = AsmBackend::new(TargetX86_64);
        let asm_code = backend.generate(ir_stream);
        utils::compiler_output(utils::format(asm_code.as_slice()), PathBuf::from("build/asm/out.asm"));
    }
}
