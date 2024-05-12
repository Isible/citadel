#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    
    use bumpalo::Bump;
    use citadel_irparser::{IRLexer, IRParser};

    use crate::experimental::{
        api::Backend,
        asm::{util, AsmBackend, TargetX86_64},
    };

    #[test]
    fn test_asm_compiler() {
        let backend = AsmBackend::new(TargetX86_64);
        let path = "tests/main.chir";
        let file_content = fs::read(path).unwrap();
        let lexer = IRLexer::new(std::str::from_utf8(&file_content).unwrap());
        let arena = Bump::new();
        let mut parser = IRParser::new(&lexer, &arena);
        let asm_code = backend.generate(parser.parse_program());
        util::compiler_output(util::format(asm_code), PathBuf::from("tests/out/out.asm"));
    }
}
