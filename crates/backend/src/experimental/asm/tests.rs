#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use citadel_frontend::ir::IRStmt;
    use citadel_irparser::{lexer::Lexer, parser::Parser};

    use crate::experimental::{
        api::Backend,
        asm::{util, AsmBackend, TargetX86_64},
    };

    #[test]
    fn test_asm_compiler() {
        let backend = AsmBackend::new(TargetX86_64);
        dbg!(gen_ir_stream(&"tests/main.chir".into()));
        let asm_code = backend.generate(gen_ir_stream(&"tests/main.chir".into()));
        util::compiler_output(util::format(asm_code), PathBuf::from("tests/out/out.asm"));
    }

    fn gen_ir_stream(path: &PathBuf) -> Vec<IRStmt> {
        let file_content = fs::read(path).unwrap();
        let lexer = Lexer::new(std::str::from_utf8(&file_content).unwrap());
        let mut parser = Parser::new(&lexer);
        parser.parse_program()
    }
}
