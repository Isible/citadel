#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    
    use citadel_irparser::{lexer::Lexer, parser::Parser};

    use crate::experimental::{
        api::Backend,
        asm::{util, AsmBackend, TargetX86_64},
    };

    #[test]
    fn test_asm_compiler() {
        let backend = AsmBackend::new(TargetX86_64);
        let path = "tests/main.chir";
        let file_content = fs::read(path).unwrap();
        let lexer = Lexer::new(std::str::from_utf8(&file_content).unwrap());
        let mut parser = Parser::new(&lexer);
        let asm_code = backend.generate(parser.parse_program());
        util::compiler_output(util::format(asm_code), PathBuf::from("tests/out/out.asm"));
    }
}
